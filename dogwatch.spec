%global debug_package %{nil}

Name: dogwatch
Version: 1.0.0
Release: 0%{?dist}
License: MIT
Summary: Prevent your machine from sleeping using dbus

# Clone this repo
# cd dogwatch && tito build --tgz
Source0: %{name}-%{version}.tar.gz

BuildRequires: rust cargo

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
