# Rustdoor
[![build](https://travis-ci.org/talbor49/rustdoor.svg?branch=master)](https://travis-ci.org/talbor49/rustdoor)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/talbor49/rustdoor/branch/master/graph/badge.svg)](https://codecov.io/gh/talbor49/rustdoor)

Rusty backdoor


Should be able to:
1. Run
2. Connect
3. Run commands
4. Upload files
5. Download files



Also:
1. Never crash
2. Be able to return to normal state



TODO by order:
* Implement better messages protocol (type+size+data) V
* Implement controller V
* Implement tests
* Make error handling better - https://rust-lang-nursery.github.io/cli-wg/tutorial/errors.html
* Report errors as a message to client
* Implement more messages
* Implement logging
* Encryption for logs, communication
* Add CI
* Implement CLI controller
