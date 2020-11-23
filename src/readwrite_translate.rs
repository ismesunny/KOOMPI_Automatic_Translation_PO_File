use csv;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

// deserializing api response from google
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Translated {
    translatedText: String,
}

#[derive(Deserialize)]
struct Translations {
    translations: Vec<Translated>,
}
#[derive(Deserialize)]
struct Ip {
    data: Translations,
}

#[derive(Debug, Serialize, Clone)]
struct Record {
    msgid: String,
    msgid_plural: String,
}
#[derive(Debug, Serialize)]
struct RecordWrite {
    msgid: String,
    msgid_plural: String,
    flags: String,
    references: String,
    #[serde(rename = "extractedComments")]
    extracted_comments: String,
    comments: String,
    #[serde(rename = "msgstr[0]")]
    msgstr0: String,
    #[serde(rename = "msgstr[1]")]
    msgstr1: String,
}
struct JSONPointer {
    segments: Vec<String>,
    segments_ac: Vec<String>,
}
fn build_json_pointer(s: Vec<String>) -> JSONPointer {
    JSONPointer {
        //replace  before translate
        segments: s
            .iter()
            .map(|x| {
                x.replace("_", "")
                    .replace(" & ", "")
                    .replace("><", "zxlessgreaterxz")
                    .replace(">/<", "zxlesslinegreaterxz")
                    .replace("&", "")
                    .replace("%", "zxpercentxz")
                    // .replace("\"", "zxdbqoutxz")
                    .replace("</", "zxlessbsxz")
                    // .replace(" <", "zxlessxz")
                    .replace(" <", ". !/zxlessspxz")
                    .replace("<", ". !/zxlessxz")
                    .replace("> ", "zxgreaterspxz/! ")
                    .replace(">", "zxgreaterxz/! ")
            })
            .collect(),
        //replace  after translate
        segments_ac: s
            .iter()
            .map(|x| {
                x.replace("_", "")
                    .replace("zxlessgreaterxz", "><")
                    .replace("zxlesslinegreaterxz", ">/<")
                    .replace("zxpercentxz", "%")
                    // .replace(" / zxlessxz", "<")
                    .replace("&quot;", "\"")
                    .replace("zxlessbsxz", "</")
                    .replace("។ ! / zxlessspxz", " <")
                    .replace("។ ! / zxlessxz", "<")
                    .replace(" ! / zxlessspxz", " <")
                    .replace(" ! / zxlessxz", "<")
                    .replace("zxgreaterspxz /! ", "> ")
                    .replace("zxgreaterxz /! ", ">")
                    .replace("< / ", "</")
                    .replace("< ", "<")
                    .replace(" >", ">")
                    //replace khmer word
                    .replace("កណ្តុរ", "ម៉ៅ")
            })
            .collect(),
    }
}

fn readcsv(first_read_csv: String) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();

    //read_data
    let mut rdr = csv::Reader::from_path(first_read_csv).unwrap();
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        records.push(Record {
            msgid: record["msgid"].to_string(),
            msgid_plural: record["msgid_plural"].to_string(),
        });
    }
    records
}
fn writecsv(
    msg_str: Vec<String>,
    msg_p_str: Vec<String>,
    read_tran: String,
    write_tran: String,
) -> Result<(), Box<dyn Error>> {
    let p_ac = build_json_pointer(msg_str);

    println!("write {:?}", p_ac.segments_ac);
    //read
    let mut rdr = csv::Reader::from_path(read_tran)?;
    let mut w_msgid = vec![];
    let mut w_msgid_plural = vec![];
    let mut w_flags = vec![];
    let mut w_references = vec![];
    let mut w_extracted_comments = vec![];
    let mut w_comments = vec![];
    let mut w_msgstr = vec![];

    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result?;
        w_msgid.push(record["msgid"].clone());
        w_msgid_plural.push(record["msgid_plural"].clone());
        w_flags.push(record["flags"].clone());
        w_references.push(record["references"].clone());
        w_extracted_comments.push(record["extractedComments"].clone());
        w_comments.push(record["comments"].clone());
        w_msgstr.push(record["msgstr[0]"].clone());
    }
    let mut wtr = csv::Writer::from_path(write_tran)?;

    for (((((((a, b), c), d), e), f), g), h) in p_ac
        .segments_ac
        .iter()
        .zip(w_msgid)
        .zip(w_msgid_plural)
        .zip(w_flags)
        .zip(w_references)
        .zip(w_extracted_comments)
        .zip(w_comments)
        .zip(msg_p_str)
    {
        wtr.serialize(RecordWrite {
            msgid: b.to_string(),
            msgid_plural: c.to_string(),
            flags: d.to_string(),
            references: e.to_string(),
            extracted_comments: f.to_string(),
            comments: g.to_string(),
            msgstr0: a.to_string(),
            msgstr1: h.to_string(),
        })?;
    }
    wtr.flush()?;
    Ok(())
}
pub fn main() {
    let input_csv = String::from("data/export_ntran_csv/file.csv");
    let output_tran_csv = String::from("data/import_ntran_csv/file.csv");

    let records = readcsv(input_csv.clone());

    let mut data_msgid: Vec<String> = Vec::new();
    let mut data_msgid_p: Vec<String> = Vec::new();

    let mut body = HashMap::new();
    let client = reqwest::blocking::Client::new();

    let source = String::from("en"); //source language
    let target = String::from("km"); //target language

    for j in records.iter() {
        data_msgid.push(j.msgid.to_string());
    }

    for i in records.iter() {
        data_msgid_p.push(i.msgid_plural.to_string());
    }
    println!("data before replace {:?}", data_msgid);
    let p = build_json_pointer(data_msgid.clone());

    println!("data after replace {:?}", p.segments);

    let mut store_msg = vec![];
    let mut store_msg_p = vec![];
    let string_null = String::from("");

    let mut _count = 0;

    //loop translate msgid word
    for i in data_msgid_p.iter() {
        if i == "" {
            println!("empty i {}", i.len());
            store_msg_p.push(string_null.to_string());
            println!("empty store_msg i {:?}", store_msg_p);
            continue;
        }
        let url = translation(i.to_string(), source.clone(), target.clone());
        body.insert("source", source.clone());
        body.insert("target", target.clone());
        body.insert("q", i.clone());

        let res: Result<Ip, reqwest::Error> =
            client.post(&url.clone()).json(&body).send().unwrap().json();
        match res {
            Ok(res) => store_msg_p.push(res.data.translations[0].translatedText.to_string()),
            Err(_) => println!("API has problem!. Please refresh your google api key."),
        }
    }

    //loop translate msgid word
    for j in p.segments.iter() {
        let url = translation(j.to_string(), source.clone(), target.clone());
        body.insert("source", source.clone());
        body.insert("target", target.clone());
        body.insert("q", j.clone());

        let res: Result<Ip, reqwest::Error> =
            client.post(&url.clone()).json(&body).send().unwrap().json();
        match res {
            Ok(res) => store_msg.push(res.data.translations[0].translatedText.to_string()),
            Err(_) => println!("API has problem!. Please refresh your google api key."),
        }
    }
    writecsv(store_msg, store_msg_p, input_csv, output_tran_csv).unwrap();
}
pub fn translation(v: String, source: String, target: String) -> String {
    let api_key = String::from("GOOGLE_API_KEY");

    let base_url = "https://translation.googleapis.com/language/translate/v2";
    format!(
        "{}{}{}{}{}",
        base_url,
        format!("?q={}", v).to_string(),
        format!("{}{}", "&source=", source),
        format!("{}{}", "&target=", target),
        format!("&key={}", api_key),
    )
}
