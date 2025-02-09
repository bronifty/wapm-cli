use crate::data::lock::lockfile_command;
use crate::data::lock::lockfile_command::LockfileCommand;
use crate::data::lock::lockfile_module::LockfileModule;
use crate::data::manifest::Manifest;
use crate::dataflow::lockfile_packages::{LockfilePackage, LockfilePackages};
use crate::dataflow::PackageKey;
use std::collections::hash_map::HashMap;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Could not extract commands from manifest. {0}")]
    CouldNotExtractCommandsFromManifest(lockfile_command::Error),
}

pub struct LocalPackage<'a> {
    pub key: PackageKey<'a>,
    pub data: LockfilePackage,
}

impl<'a> LocalPackage<'a> {
    pub fn new_from_local_package_in_manifest(manifest: &'a Manifest) -> Result<Self, Error> {
        let package_name = manifest.package.name.as_str();
        let package_version = &manifest.package.version;
        let modules = manifest
            .module
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                LockfileModule::from_local_module(
                    &manifest.base_directory_path,
                    package_name,
                    package_version,
                    &m,
                )
            })
            .collect();
        let commands = manifest
            .command
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|c| LockfileCommand::from_command(package_name, package_version.clone(), &c))
            .collect::<Result<Vec<LockfileCommand>, lockfile_command::Error>>()
            .map_err(Error::CouldNotExtractCommandsFromManifest)?;
        let key = PackageKey::new_registry_package(package_name, package_version.clone());
        let data = LockfilePackage { modules, commands };
        Ok(LocalPackage { key, data })
    }
}

impl<'a> From<LocalPackage<'a>> for LockfilePackages<'a> {
    fn from(local: LocalPackage<'a>) -> LockfilePackages<'a> {
        let mut packages = HashMap::new();
        packages.insert(local.key, local.data);
        LockfilePackages { packages }
    }
}
