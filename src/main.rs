use calamine::{open_workbook, Reader, Xlsx};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let excel_path = "/Users/quantum/Desktop/generator-rs/HPC_accounts.xlsx";
    let destination_path = "export.sh";
    let accounting = "chem_workshop";
    let max_jobs = 2;
    let max_cpu = 2;

    println!("Scanning {:?}", excel_path);

    let mut excel: Xlsx<_> = open_workbook(excel_path)?;

    let mut write_buf = String::new();

    write_buf.push_str(&format!(
        "sacctmgr add account {} --immediate\n",
        accounting
    ));

    let names = excel.sheet_names().to_vec();

    for sheet_name in names {
        if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {
            let mut rows = r.rows().into_iter();
            let header = rows.nth(0).unwrap();
            let username_index = header
                .into_iter()
                .position(|v| v == "Username" || v == "username")
                .expect("Unable to find Username column");

            // let password_index = header
            //     .into_iter()
            //     .position(|v| v == "Password")
            //     .expect("Unable to find Password column");

            for row in rows {
                let username = &row[username_index];
                let password = "chemist";

                let template = format!(
                    r#"
useradd -e 2020-08-26 -m {uname}
echo "{password}" | passwd --stdin {uname}
mkdir -p /mnt/lustre/{uname}
chown -R {uname}:{uname} /mnt/lustre/{uname}/
sacctmgr create user name={uname} DefaultAccount={accounting} Partition=normal --immediate
sacctmgr create user name={uname} DefaultAccount=gpu72 Partition=short --immediate
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
    }

    println!("Writing output to {:?}", destination_path);

    fs::write(destination_path, write_buf).expect("Unable to write output file");

    Ok(())
}
