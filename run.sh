#!/usr/bin/env bash

LOG_FILE=$"test-bed/log.txt"
DIAGNOSTICS_FILE=$"test-bed/diagnostics.txt"
LOSS_LOGIC_DATA_FILE=$"LossData_Logic.xlsx"
AS_ON_DATE=$2
OUTPUT_FILE=$"test-bed/Output.txt"
OUT_FILE="test-bed/Outfile.txt"

cargo run  -- \
--diagnostics-log-file ${DIAGNOSTICS_FILE} \
--log-file ${LOG_FILE} \
--log-level trace \
--loss-logic-data ${LOSS_LOGIC_DATA_FILE} \
--gross-loss-id "1001" \
--credit-loss-id "2001" \
--recovery-other-than-credit-loss-id "3001" \
--net-loss-id "4001" \
--external-fraud-id "5001" \
--as-on-date 23-07-2023 \
--operational-loss-id "6001" \
--output-file ${OUTPUT_FILE} \
--finnone-master-sheet-name "Input File" \
--diagnostics-flag false 

grep "6001" Output.txt|awk -v var="$2" -F'|' '$1==6001{print var"|INR|"$2"|"$2}' $OUTPUT_FILE >$OUT_FILE
