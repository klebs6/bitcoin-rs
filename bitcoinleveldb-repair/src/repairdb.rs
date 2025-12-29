// ---------------- [ File: bitcoinleveldb-repair/src/repairdb.rs ]
crate::ix!();

pub fn repairdb(dbname: &String, options: &Options) -> crate::Status {
    trace!(dbname = %dbname, "repairdb: start");

    let mut repairer: Repairer = Repairer::new(dbname, options);
    let status = repairer.run();

    debug!(
        dbname = %dbname,
        ok = status.is_ok(),
        status = %status.to_string(),
        "repairdb: done"
    );

    status
}
