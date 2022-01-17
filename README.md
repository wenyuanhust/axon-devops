# use hardhat tool  to test and compile solitity constract
#### axon uniswap constract case:
```shell
$ cd ~
$ git clone https://github.com/nervosnetwork/axon-devops.git
$ cd axon-devops/uni-test
$ npm install
```
## start a hardhat evm node
```shell
$ npx hardhat node
```
#
## compile all constracts
```shell
$ npx hardhat compile
```
#
## test uniswap constracts
```shell
$ npx hardhat run scripts/test-uniswap-script.js
```
#

## publish constracts to axon
```shell
$ cd ../uni-contract-deploy
$ npm install
```
modify network setting in the config.yml as following configuration code is shown
```shell
$ node templates/deploy.js
```

# deploy uni interface
```shell
$ cd uni-interface-deploy && make deploy 
```

## metamask network
name: GÖRLI
chain id: 5



## Reference
[Reference Links](https://segmentfault.com/a/1190000040401731)
