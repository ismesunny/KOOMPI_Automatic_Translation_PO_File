use csv;
use htmlstream;
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
struct WordReplaceBeforeTran {
    segments_before_tran: String,
}
struct WordReplaceHtml {
    segments_html: Vec<String>,
}
struct WordReplaceAfterTran {
    segments_after_tran: Vec<String>,
}
fn replacing_before_tran(before_tran: String) -> WordReplaceBeforeTran {
    WordReplaceBeforeTran {
        segments_before_tran: before_tran
            .replace("_", "")
            .replace(" & ", "")
            .replace("&", "")
            .replace("%", "zzpercentzz")
            .replace(">/<", "zzlesslinegreaterzz"),
    }
}
fn replacing_html(replace_html: Vec<String>) -> WordReplaceHtml {
    WordReplaceHtml {
        segments_html: replace_html
            .iter()
            .map(|x| x.replace("\"", "zxdbquotxz"))
            .collect(),
    }
}
fn replacing_after_tran(after_tran: Vec<String>) -> WordReplaceAfterTran {
    WordReplaceAfterTran {
        segments_after_tran: after_tran
            .iter()
            .map(|x| {
                x.replace("> <", "><")
                    .replace("zzpercentzz", "%")
                    .replace("&quot;", "\"")
                    .replace("> + <", ">+<")
                    .replace("> - <", ">-<")
                    .replace("> / <", ">/<")
                    .replace("zxdbquotxz", "\"")
                    //replace khmer word
                    .replace("កណ្តុរ", "ម៉ៅ")
                    .replace("ត្រីដូហ្វីន", "ដូហ្វីន")
                    .replace("៧ ស", "7z")
                    .replace("ទូក", " Ark ")
                    .replace("បង្អួច", "ផ្ទាំងកម្មវិធី")
                    //replace after khmer word replace khmer word
                    .replace(">  ", ">")
                    .replace("  <", "<")
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
    let p_ac = replacing_after_tran(msg_str);

    println!("after replace {:?}", p_ac.segments_after_tran);
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
        .segments_after_tran
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
    let input_csv = String::from("data/csv_cache/non_translate_csv/file.csv");
    let output_tran_csv = String::from("data/csv_cache/translated_csv/file.csv");

    let records = readcsv(input_csv.clone());

    let mut data_msgid: Vec<String> = Vec::new();
    let mut data_msgid_p: Vec<String> = Vec::new();
    let mut store_msg = vec![];
    let mut store_msg_p = vec![];
    let string_null = String::from("");

    for i in records.iter() {
        data_msgid_p.push(i.msgid_plural.to_string());
    }

    for j in records.iter() {
        data_msgid.push(j.msgid.to_string());
    }
    let i_afterreplace = replacing_html(data_msgid_p.clone());
    let j_afterreplace = replacing_html(data_msgid.clone());
    let mut _count = 0;

    //loop translate msgid word
    for i in i_afterreplace.segments_html.iter() {
        if i == "" {
            store_msg_p.push(string_null.to_string());
            continue;
        }
        let msg_p = translate_text_html(&i);
        store_msg_p.push(msg_p);
        println!("msg plural word {:?}", store_msg_p);
    }
    for j in j_afterreplace.segments_html.iter() {
        let msg_tran = translate_text_html(&j);
        store_msg.push(msg_tran);
        println!("msg word {:?}", store_msg)
    }
    writecsv(store_msg, store_msg_p, input_csv, output_tran_csv).unwrap();
}
fn translate_text_html(s_text: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let source = String::from("en"); //source language
    let target = String::from("km"); //target language

    let mut body = HashMap::new();
    let mut inner_text: Vec<(usize, String)> = Vec::new();
    let mut inner_tag: Vec<(usize, String)> = Vec::new();
    let mut store_vec: Vec<String> = Vec::new();

    for (idx, (_pos, tag)) in htmlstream::tag_iter(s_text).enumerate() {
        if tag.html.starts_with("<") && tag.html.ends_with(">") {
            inner_tag.push((idx, tag.html));
            println!("inner_tag {:?}", inner_tag);
        } else {
            inner_text.push((idx, tag.html));
            println!("inner_text {:?}", inner_text);
        }
    }
    for (_pos, text) in inner_text {
        println!("data before replace {:?}", text);
        let p = replacing_before_tran(text.clone());
        println!("data after replace {:?}", p.segments_before_tran);
        let url = translation(
            p.segments_before_tran.clone(),
            source.clone(),
            target.clone(),
        );
        body.insert("source", source.clone());
        body.insert("target", target.clone());
        body.insert("q", p.segments_before_tran.clone());

        let res: Result<Ip, reqwest::Error> =
            client.post(&url.clone()).json(&body).send().unwrap().json();
        match res {
            Ok(res) => store_vec.push(res.data.translations[0].translatedText.to_string()),
            Err(_) => println!("API has problem!. Please refresh your google api key."),
        }
    }
    inner_tag
        .into_iter()
        .for_each(|(pos, tag)| store_vec.insert(pos, tag));
    let joined_str = store_vec.join(" ");
    println!("string word has joined {}", joined_str);
    joined_str
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
