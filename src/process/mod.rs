use calamine::open_workbook;
use calamine::{open_workbook_auto, Reader, Sheets, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::write;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead, BufReader};

pub fn lossdata_pp(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let mut output = File::create(config_params.output_file()).expect("Failed to create a file");
    let mut sum_gross_loss: f64 = 0.0;
    let mut sum_credit_loss: f64 = 0.0;
    let mut sum_recovery_loss: f64 = 0.0;
    let mut sum_net_loss: f64 = 0.0;
    let mut sum_external_fraud: f64 = 0.0;
    let mut sum_operational_loss: f64 = 0.0;
    let mut gross_loss_id: String = config_params.gross_loss_id().to_string();
    let mut credit_loss_id: String = config_params.credit_loss_id().to_string();
    let mut recovery_loss_id: String = config_params.recovery_other_than_credit_loss_id().to_string();
    let mut net_1oss_id: String = config_params.net_loss_id().to_string();
    let mut operational_loss_id: String = config_params.operational_loss_id().to_string();
    let mut external_fraud_id: String = config_params.external_fraud_id().to_string();
    let mut as_on_date: NaiveDate = config_params.as_on_date;
    let mut as_on_date_string = as_on_date.to_string();
    let mut lossdatalogicreader: Xlsx<_> =
        open_workbook(config_params.loss_logic_data()).expect("unable to read excel");

    if let Some(Ok(reader)) =
        lossdatalogicreader.worksheet_range(config_params.loss_logic_data_sheet_name())
    {
        for row in reader.rows().skip(1) {
            acc_enc += 1;
            let mut gross_float: f64 = row[9].to_string().parse().unwrap();
            sum_gross_loss = sum_gross_loss + gross_float;
            if row[14].to_string() == "ILET21" {
                sum_credit_loss = sum_credit_loss + gross_float;
            }
            if row[14].to_string() != "ILET21" {
                let mut recovery_loss_float: f64 = row[10].to_string().parse().unwrap();
                sum_recovery_loss = sum_recovery_loss + recovery_loss_float;
            }
            if row[14].to_string() == "ILET22" {
                let mut external_fraud_float: f64 = row[11].to_string().parse().unwrap();
                sum_external_fraud = sum_external_fraud + external_fraud_float;
            }
            acc_proc += 1;
            ip_amt = ip_amt + gross_float;
            op_amt = op_amt + gross_float;
        }
    }
    //let mut final_output_text_gross_loss = format!("{}\n", as_on_date.format("%d-%m-%Y"));

    /*final_output_text = (as_on_date.format("%d-%m-%Y").to_string())
        + &(final_output_text + &format!("{}|{}\n", gross_loss_id, sum_gross_loss));

    final_output_text = (as_on_date.format("%d-%m-%Y").to_string())
        + &(final_output_text + &format!("{}|{}\n", credit_loss_id, sum_credit_loss));

    final_output_text = (as_on_date.format("%d-%m-%Y").to_string())
        + &(final_output_text + &format!("{}|{}\n", recovery_loss_id, sum_recovery_loss));*/

    sum_net_loss = sum_gross_loss - (sum_credit_loss + sum_recovery_loss);
    /*final_output_text = (as_on_date.format("%d-%m-%Y").to_string())
        + &(final_output_text + &format!("{}|{}\n", net_1oss_id, sum_net_loss));

    final_output_text =
        final_output_text + &format!("{}|{}\n", external_fraud_id, sum_external_fraud);*/

    sum_operational_loss = sum_net_loss - sum_external_fraud;
   // final_output_text = (as_on_date.format("%d-%m-%Y").to_string())+ &(final_output_text + &format!("{}|{}\n", operational_loss_id, sum_operational_loss));

    let final_output_text=format!("{}|{}\n{}|{}\n{}|{}\n{}|{}\n{}|{}\n{}|{}\n",gross_loss_id,sum_gross_loss,credit_loss_id,sum_credit_loss,recovery_loss_id,sum_recovery_loss,net_1oss_id,sum_net_loss,operational_loss_id,sum_operational_loss,external_fraud_id,sum_external_fraud);    
    output.write_all(&final_output_text.as_bytes()).expect("Failed to write to a file");


    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(config_params.output_file());
}

