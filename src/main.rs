use calamine::{open_workbook, Reader, Xlsx};
use std::error::Error;
use std::fs;
use structopt::StructOpt;

mod args;

use crate::args::GeneratorConfig;
fn main() -> Result<(), Box<dyn Error>> {
    let args = GeneratorConfig::from_args();

    let excel_path = args.excel_path.expect("Please specify excel path");
    let destination_path = args.destination_path;
    let accounting = args.accounting.expect("Please specify the accounting");
    let max_jobs = args.max_jobs.expect("Please specify the max jobs");
    let max_cpu = args.max_cpu.expect("Please specify the max cpu");

    match excel_path.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file"),
    }

    println!("Scanning {:?}", excel_path);

    let mut excel: Xlsx<_> = open_workbook(excel_path)?;

    let mut write_buf = String::new();

    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let mut rows = r.rows().into_iter();
        let header = rows.nth(0).unwrap();
        let username_index = header
            .into_iter()
            .position(|v| v == "Username")
            .expect("Unable to find Username column");

        let password_index = header
            .into_iter()
            .position(|v| v == "Password")
            .expect("Unable to find Password column");

        for row in rows {
            let username = &row[username_index];
            let password = &row[password_index];

            let template = format!(
                r#"
useradd -e 2020-05-10 -m {uname}
echo "{password}" | passwd --stdin {uname}
mkdir -p /mnt/lustre/{uname}
chown -R {uname}:{uname} /mnt/lustre/{uname}/
sacctmgr create user name={uname} DefaultAccount={accounting} --immediate
sacctmgr modify user {uname} set GrpTRES=cpu={max_cpu} --immediate
sacctmgr modify user {uname} set MaxJobs={max_jobs} --immediate


            "#,
                uname = username,
                password = password,
                accounting = accounting,
                max_jobs = max_jobs,
                max_cpu = max_cpu
            );

            write_buf.push_str(template.as_str());
        }
    };

    println!("Writing output to {:?}", destination_path);

    fs::write(destination_path, write_buf).expect("Unable to write output file");

    Ok(())
}
