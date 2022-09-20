%global debug_package %{nil}
%{!?pkgrevision: %global pkgrevision 1}
%define installpath /usr/edb/efm-api-node-state
%define _unpackaged_files_terminate_build 0

Name:      efm-api-node-state
Version:   %{pkgversion}
Release:   %{pkgrevision}%{?dist}
Summary:   HTTP service and REST API exposing the state of the current EFM node
License:   BSD
BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root-%(%{__id_u} -n)


%description
HTTP service and REST API exposing the state of the current EFM node

%prep

%build
cargo build --release

%install
%{__install} -d %{buildroot}/%{installpath}/bin
%{__install} -d %{buildroot}/usr/edb/efm-api-node-state/scripts
%{__install} -d %{buildroot}/etc/edb/efm-api-node-state
%{__install} -d %{buildroot}/lib/systemd/system

%{__install} /workspace/target/release/efm-api-node-state %{buildroot}/%{installpath}/bin/efm-api-node-state
%{__install} /workspace/scripts/efm_check_primary.sh %{buildroot}/usr/edb/efm-api-node-state/scripts/efm_check_primary.sh
%{__install} /workspace/scripts/efm_check_standby.sh %{buildroot}/usr/edb/efm-api-node-state/scripts/efm_check_standby.sh
%{__install} /workspace/scripts/efm_monitoring.sh %{buildroot}/usr/edb/efm-api-node-state/scripts/efm_monitoring.sh
%{__install} /workspace/config.toml %{buildroot}/etc/edb/efm-api-node-state/config.toml
%{__install} /workspace/systemd/efm-api-node-state.service %{buildroot}/lib/systemd/system/efm-api-node-state.service

%files
%{installpath}
/usr/edb/efm-api-node-state
/etc/edb/efm-api-node-state
%attr(-, root, root) %{installpath}/bin/efm-api-node-state
%attr(-, root, root) /usr/edb/efm-api-node-state/scripts/efm_check_primary.sh
%attr(-, root, root) /usr/edb/efm-api-node-state/scripts/efm_check_standby.sh
%attr(-, root, root) /usr/edb/efm-api-node-state/scripts/efm_monitoring.sh
%attr(0644, root, root) /etc/edb/efm-api-node-state/config.toml
%attr(-, root, root) /lib/systemd/system/efm-api-node-state.service

%preun
if [ "$1" = "0" ]; then
	/bin/systemctl stop efm-api-node-state
fi
exit 0

%changelog
* Mon Sep 12 2022 Julien Tachoires <julien.tachoires@enterprisedb.com> - 0.1.0-1
- Initial release

* Tue Sep 20 2022 Julien Tachoires <julien.tachoires@enterprisedb.com> - 0.2.0-1
- Release 0.2.0
