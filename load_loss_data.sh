sqlldr $AXIS_INTERFACE_AE\
control=$SCRIPTS/IND/loader-scripts/CARE/BULK_INSERT/prg1.ctl \
data=$/home/surya71/Projects/Rust_Backup/package/SuperDB-Batch-1/sib/pre-processors/pp-loss-data/test-bed/Outfile.txt \
log=$LOGS/IND/CARE/$1/BULK_INSERT_OPRSTALCData.log \
bad=$LOGS/IND/CARE/$1/BULK_INSERT_OPRSTALCData.bad\
silent=feedback,header

