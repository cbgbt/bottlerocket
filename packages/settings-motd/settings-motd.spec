%global _cross_first_party 1
%undefine _debugsource_packages

Name: %{_cross_os}settings-motd
Version: 0.0
Release: 0%{?dist}
Summary: settings-motd
License: Apache-2.0 OR MIT
URL: https://github.com/bottlerocket-os/bottlerocket

BuildRequires: %{_cross_os}glibc-devel

%description
%{summary}.

%prep
%setup -T -c
%cargo_prep

%build
mkdir bin
%cargo_build --manifest-path %{_builddir}/sources/Cargo.toml \
    -p settings-extension-motd

%install
install -d %{buildroot}%{_cross_bindir}
install -p -m 0755 ${HOME}/.cache/%{__cargo_target}/release/settings-extension-motd %{buildroot}%{_cross_bindir}
 
%files
%{_cross_bindir}/settings-extension-motd
