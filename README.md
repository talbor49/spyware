# Rustdoor

[![build](https://travis-ci.org/talbor49/rustdoor.svg?branch=master)](https://travis-ci.org/talbor49/rustdoor)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/talbor49/rustdoor/branch/master/graph/badge.svg)](https://codecov.io/gh/talbor49/rustdoor)

### Motivation
Spyware that spies on you, by you (for once)

### Includes
#### Backdoor
    * Connect
    * Run commands
    * Upload files
    * Download files
#### CNC server
    * Report information to the server
    * Can pull commands from server
    
#### Generic features
    * Never crash
    * Be able to return to normal state after failure
    * Cross platform - windows/linux/[android/ios]

#### TODO:
    - Implement better messages protocol (type+size+data) V
    - Implement controller V
    - Implement tests
    - Make error handling better - https://rust-lang-nursery.github.io/cli-wg/tutorial/errors.html
    - Report errors as a message to client
    - Implement more messages
    - Implement logging
    - Encryption for logs, communication
    - Add CI V
    - Implement CLI controller V
    - Find better ways to communicate instead of listening on port - maybe OOB data?
    - Document with rustdoc
    - Gather information (keylogger, location, wifi, etc)
    - Find ways to start after reboot - preferably also after format. 
