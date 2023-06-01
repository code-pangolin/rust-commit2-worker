use fvm_shared::sector::SectorID;

pub fn sector_name(sector_id: SectorID) -> String {
    format!("s-t0{}-{}", sector_id.miner, sector_id.number)
}
