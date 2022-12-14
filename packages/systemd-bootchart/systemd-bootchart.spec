Name: %{_cross_os}systemd-bootchart
Version: 234
Release: 1%{?dist}
Summary: Boot performance graphing tool
	
License:        GPLv2+ and LGPLv2+
URL:            https://github.com/systemd/systemd-bootchart
Source0:        https://github.com/systemd/systemd-bootchart/releases/download/v%{version}/systemd-bootchart-%{version}.tar.xz
 
BuildRequires: %{_cross_os}glibc-devel
BuildRequires: %{_cross_os}systemd-devel
Requires: %{_cross_os}systemd

%description
This package provides a binary which can be started during boot early boot to
capture informations about processes and services launched during bootup.
Resource utilization and process information are collected during the boot
process and are later rendered in an SVG chart. The timings for each services
are displayed separately.

%prep
%autosetup -n systemd-bootchart-%{version} -p1

%build
%cross_configure \
    --disable-man
%make_build

%install
%make_install

%files
%license LICENSE.GPL2
%license LICENSE.LGPL2.1

# These still don't work
# It seems like the install section is not respecting PREFIX for some reason...
%{_cross_libdir}/systemd/systemd-bootchart
%{_cross_datarootdir}%{_cross_sysconfdir}/systemd/bootchart.conf
%{_cross_unitdir}/systemd-bootchart.service

%changelog
