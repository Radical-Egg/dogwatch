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
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%build
%cargo_buildi -a

%install
%cargo_install -a

%files
%{_bindir}/dogwatch
