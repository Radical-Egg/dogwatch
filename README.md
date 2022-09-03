# dogwatch ![Github CI](https://github.com/Radical-Egg/dogwatch/actions/workflows/rust.yml/badge.svg)


You've been working the dogwatch for a few weeks now. It's past the point in the night where Fred Durst doesn't seem like he's joking about his career anymore, perhaps he was serious this whole time. You just need a power nap to get you through the rest of the night. Your manager has heen working remote for the past two years but he could show back up any day now. If only there was a way to keep my computer from sleeping with a single command so that you can take a quick nap.

## Usage

- Stay awake forever by running ```dogwatch```
- Stay awake for a specific amount of time (minutes) ```dogwatch --time 1```

## Building

- If you are using a distro that uses RPMs you can download the RPM in the releases tab and install it with ```rpm -ivh```
- ```git clone https://github.com/Radical-Egg/dogwatch```
- ```cd dogwatch```
- use ```cargo run``` or ```cargo build```

## Feature Roadmap

- Self Driving Screen Saver
- Fully Autonomous Self Driving Screen Saver (please don't try this at home)
- Internet wide ad blocking
- Basic calculator functionality


## Disclaimer

Fred Durst might be a cool guy I don't actually know anything about him or his career. This is my frist project in Rust. Rust seems pretty cool. I wanted to learn about Dbus and this seemed like a cool way to do that. I found some good resources for anyone who is looking into Dbus.

- [dbus-sample](https://github.com/makercrew/dbus-sample): This link is awesome and has a really good high level overview of dbus as well as a ton of useful links. There is also a sample cpp program on how to make a connection and introspect.
- [dbus-rs](https://github.com/diwic/dbus-rs): D-Bus bindings for rust
- [dbus tutorial](https://dbus.freedesktop.org/doc/dbus-tutorial.html): As you can tell by the CSS used on this webpage they would like to keep it strictly business
- [Qdbusviewer](https://doc.qt.io/qt-6/qdbusviewer.html): I didn't find out about this until after I was done but this is a great way to mess around with Dbus before you start programming something. Here is the flathub link to download this jawn [flathub link](https://flathub.org/apps/details/io.qt.qdbusviewer)
