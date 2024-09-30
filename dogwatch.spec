%global debug_package %{nil}

Name: dogwatch
Version: 1.1.0
Release: 1%{?dist}
License: MIT
Summary: Prevent your machine from sleeping using dbus

# Clone this repo
# cd dogwatch && tito build --tgz
Source0: %{name}-%{version}.tar.gz

BuildRequires: rust cargo dbus-devel

%description
Prevent your machine from sleeping using dbus-rs bindings

%prep
%autosetup

%build
cargo build --release
strip -s ./target/release/dogwatch

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/%{_bindir}
cp ./target/release/%{name} $RPM_BUILD_ROOT/%{_bindir}

%clean
rm -rf $RPM_BUILD_ROOT

%files
%{_bindir}/dogwatch

%changelog
* Sun Sep 29 2024 Unknown name 1.1.0-1
- Add functionality for Windows (luther.jaymen@gmail.com)
- Update clap requirement from 3.2.18 to 4.1.11
  (49699333+dependabot[bot]@users.noreply.github.com)

* Sun Sep 29 2024 Unknown name
- Add functionality for Windows (luther.jaymen@gmail.com)
- Update clap requirement from 3.2.18 to 4.1.11
  (49699333+dependabot[bot]@users.noreply.github.com)

* Sun Sep 29 2024 Unknown name
- Add functionality for Windows (luther.jaymen@gmail.com)
- Update clap requirement from 3.2.18 to 4.1.11
  (49699333+dependabot[bot]@users.noreply.github.com)

* Thu Sep 08 2022 egg <egg95@protonmail.com> 1.0.3-1
- updated version (egg95@protonmail.com)
- strip symbols and enable LTO (juanbono94@gmail.com)
- Update README.md (45681670+Radical-Egg@users.noreply.github.com)
- Update README.md (45681670+Radical-Egg@users.noreply.github.com)

* Wed Sep 07 2022 egg <egg95@protonmail.com> 1.0.2-1
- replaced dbus-libs with dbus-devel 

* Wed Sep 07 2022 egg <egg95@protonmail.com> 1.0.1-1
- added copr badge (45681670+Radical-Egg@users.noreply.github.com)
- Automatic commit of package [dogwatch] release [1.0.0-1].
  (egg95@protonmail.com)
- added dbus-libs build req
* Wed Sep 07 2022 egg <egg95@protonmail.com> 1.0.0-1
- new package built with tito

