use std::{ffi::OsString, process::Command};

use log::trace;
use miette::{IntoDiagnostic, Result, bail};

use super::opts::CopyOciOpts;
use crate::logging::CommandLogging;

#[derive(Debug)]
pub struct SkopeoDriver;

impl super::OciCopy for SkopeoDriver {
    fn copy_oci(&self, opts: CopyOciOpts) -> Result<()> {
        trace!("SkopeoDriver::copy_oci({opts:?})");
        let use_sudo = opts.privileged && !blue_build_utils::running_as_root();
        let mut initial_args = Vec::<OsString>::new();
        if use_sudo {
            initial_args.push("sudo".into());
            if blue_build_utils::has_env_var(blue_build_utils::constants::SUDO_ASKPASS) {
                initial_args.push("-A".into());
                initial_args.push("-p".into());
                initial_args.push(
                    format!(
                        "Password is required to copy {source} to {dest}",
                        source = opts.src_ref,
                        dest = opts.dest_ref,
                    )
                    .into(),
                );
            }
        }
        if opts.podman_unshare {
            initial_args.push("podman".into());
            initial_args.push("unshare".into());
        }
        initial_args.push("skopeo".into());
        let mut initial_args = initial_args.into_iter();

        let mut skopeo_cmd = Command::new(initial_args.next().unwrap());
        skopeo_cmd.args(initial_args);
        skopeo_cmd.arg("copy");
        skopeo_cmd.arg("--all");
        if opts.retry_count != 0 {
            skopeo_cmd.arg(format!("--retry-times={}", opts.retry_count));
        }
        skopeo_cmd.arg(opts.src_ref.to_os_string());
        skopeo_cmd.arg(opts.dest_ref.to_os_string());
        trace!("{skopeo_cmd:?}");

        let status = skopeo_cmd
            .build_status(
                opts.dest_ref.to_string(),
                format!("Copying {} to", opts.src_ref),
            )
            .into_diagnostic()?;

        if !status.success() {
            bail!("Failed to copy {} to {}", opts.src_ref, opts.dest_ref);
        }

        Ok(())
    }
}
