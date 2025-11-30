use crate::{PitouDrive, PitouDriveKind, PitouFilePath};
use std::path::PathBuf;
use sysinfo::{Disk, DiskKind, Disks};

impl PitouDrive {
    pub fn get_drives() -> Vec<Self> {
        let mut drives = Disks::new_with_refreshed_list()
            .into_iter()
            .map(|d| Self::to_drive(d))
            .collect::<Vec<_>>();
        drives.sort_unstable_by(|a, b| a.mount_point().as_bytes().cmp(&b.mount_point().as_bytes()));
        drives
    }

    fn to_drive(disk: &Disk) -> Self {
        let mount_point = PitouFilePath::from_pathbuf(PathBuf::from(disk.mount_point()));
        let is_removable = disk.is_removable();
        let total_space = disk.total_space();
        let free_space = disk.available_space();
        let kind = match disk.kind() {
            DiskKind::HDD => PitouDriveKind::HDD,
            DiskKind::SSD => PitouDriveKind::SSD,
            DiskKind::Unknown(_) => PitouDriveKind::Unknown,
        };
        let name = disk
            .name()
            .to_str()
            .map(|v| v.to_owned())
            .unwrap_or_default();

        PitouDrive {
            mount_point,
            total_space,
            free_space,
            is_removable,
            kind,
            name,
        }
    }
}
