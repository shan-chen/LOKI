LOKI is used to reproduced the following bugs and can successfully reproduce 10 of them. Bug #3 and Bug #8 are due to data race which can be hard to reproduce.

| Number | Platform    | Link                                                       |
|--------|-------------|------------------------------------------------------------|
| 1      | Go-Ethereum | https://github.com/ethereum/go-ethereum/issues/25953       |
| 2      | Go-Ethereum | https://github.com/ethereum/go-ethereum/issues/25787       |
| 3      | Go-Ethereum | https://github.com/ethereum/go-ethereum/issues/25868       |
| 4      | Fabric      | https://jira.hyperledger.org/projects/FAB/issues/FAB-18239 |
| 5      | Fabric      | https://jira.hyperledger.org/projects/FAB/issues/FAB-18535 |
| 6      | Fabric      | https://jira.hyperledger.org/projects/FAB/issues/FAB-14470 |
| 7      | Diem        | https://github.com/diem/diem/issues/8704                   |
| 8      | Diem        | https://github.com/diem/diem/issues/8423                   |
| 9      | Diem        | https://github.com/diem/diem/issues/7643                   |
| 10     | FISCO-BCOS  | https://github.com/FISCO-BCOS/FISCO-BCOS/issues/2101       |
| 11     | FISCO-BCOS  | https://github.com/FISCO-BCOS/FISCO-BCOS/issues/2206       |
| 12     | FISCO-BCOS  | https://github.com/FISCO-BCOS/FISCO-BCOS/issues/2254       |
