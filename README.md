# network_doctor
Simple project in rust for a CLI tool that made some basic diagnosis of network connections in you local computer.

# Hello World!

**Hello everybody!**

## The Project
So, this is a project that it was always pending in my mind.
I worked in a ISP company for 3 and half years, and I thought
that a tool that make the basics status check for a client
that has problems with is connection can be useful.

## Why Rust?
Recently I finished **The Book** and I reading and coding 
with **Command Line Rust** (it is a great guide!), so this
is a great opportunity the really learn Rust whit a real
project.

I'm sure that I could make this project 10x faster in Go
and will by equally practical for an end user, but no!
I want to do this in Rust and only Rust.

# My rules


## No IA
I really think that IA is a great tool, I for the present moment
I have pending to create my own IA agent, but for really
learn Rust a solve problems relate to the system programming
I decide to learn in the old and hard way.

This is, making a lot of mistakes, the code not compiling,
reading the docs and StackOverflow (or what is left).

## Only VIM or NVIM
This tool is one that I always have pending.
But no anymore!
I'm using (N)VIM without any extension, more like a notepad
editor. In this regard, I'm following the recommendations 
of the Old Guard to really learn and tune my coding abilities.


# Objectives
## General
Construct a useful CLI tool to realize an acceptable diagnosis
of internet problems a connection for an end user.

## Specifics
### Ping
Implement a ping command that point to some ipv4, ipv6, and DNS.

### Tracert
We need to detect what IP addresses or DNS do not resolver and 
then make a tracert/traceroute of that resource.

### ipconfig/ifconfig
For the moment I think that I must use this command to extract 
the IP of the local router and make a Ping to the equip in order to discard malfunctioning of the hardware.

### nslookup
If necessary, check the correct translation of the dns and also 
check the IP addresses returned.

### neststat
Get the status of used ports at the moment of analisys.

### MAC (probably optional)
Maybe useful in particular cases.

### arp
Check IP conflicts in the same network.

### pathping / mtr
Maybe optional.

### uptime
Check for drop in performance in case that the pc has much 
time working (specially Windows).

### ip route
Check for bad VPN config.
(README under construction...)
