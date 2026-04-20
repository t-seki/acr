# Changelog

## [0.4.4](https://github.com/t-seki/acr/compare/v0.4.3...v0.4.4) (2026-04-20)


### Bug Fixes

* **ci:** drop draft release so release-please creates the tag too ([d35982f](https://github.com/t-seki/acr/commit/d35982f9929ee55fb73cd102ce390b27921e0412))
* **ci:** drop release-please draft mode so it creates the tag too ([1ab94db](https://github.com/t-seki/acr/commit/1ab94dbb998585787711ca2548b61e6b9b79ea54))
* **ci:** integrate release-please with cargo-dist via draft releases ([696cced](https://github.com/t-seki/acr/commit/696cceda5c9d07ebb181b895828e7171b15780a6))
* **ci:** integrate release-please with cargo-dist via draft releases ([782bf55](https://github.com/t-seki/acr/commit/782bf55feec79ad8814de3b3370b333c4ecfae3a))

## [0.4.3](https://github.com/t-seki/acr/compare/v0.4.2...v0.4.3) (2026-04-20)


### Bug Fixes

* **ci:** let cargo-dist own the GitHub Release, not release-please ([b219699](https://github.com/t-seki/acr/commit/b21969971d8386e2cf2f7f42f8ff9ed59bb0dafb))
* **ci:** let cargo-dist own the GitHub Release, not release-please ([f036b6d](https://github.com/t-seki/acr/commit/f036b6d323c5588d0e2f9560961ba44721dc817d))

## [0.4.2](https://github.com/t-seki/acr/compare/v0.4.1...v0.4.2) (2026-04-20)


### Bug Fixes

* **ci:** use PAT for release-please so tag pushes trigger downstream workflows ([9f7a4ba](https://github.com/t-seki/acr/commit/9f7a4baa22f9e7f7f7a59028143b457fb3616474))
* **ci:** use PAT for release-please so tag pushes trigger downstream workflows ([a198cfa](https://github.com/t-seki/acr/commit/a198cfaefa9b2ae0658252c00a4e7988385c792a))

## [0.4.1](https://github.com/t-seki/acr/compare/v0.4.0...v0.4.1) (2026-04-20)


### Bug Fixes

* **ci:** match release-please tag pattern and allow manual publish ([e5ae6d0](https://github.com/t-seki/acr/commit/e5ae6d0bce7dd9b74ad99b91a7060a669fde72ce))
* **ci:** match release-please tag pattern and allow manual publish ([98ee720](https://github.com/t-seki/acr/commit/98ee720be1b4a1954066d4c79c9603ddeda9e69e))

## [0.4.0](https://github.com/t-seki/acr/compare/acr-cli-v0.3.4...acr-cli-v0.4.0) (2026-04-20)


### Features

* add --at flag to schedule contest workspace creation ([ba28a8c](https://github.com/t-seki/acr/commit/ba28a8c2bb0051698084f797c89ca133b149d00f))
* add --at flag to schedule contest workspace creation ([4e63562](https://github.com/t-seki/acr/commit/4e63562d74701c67159203663ac4ccf4850de628))
* add --force flag to submit command ([cdf13a5](https://github.com/t-seki/acr/commit/cdf13a5073e4aa738e628e1fda618611199527ae))
* add --force flag to submit command ([69e7e7d](https://github.com/t-seki/acr/commit/69e7e7d590f60ccfa17f5813446d6f603a40e5a0)), closes [#28](https://github.com/t-seki/acr/issues/28)
* add `acr virtual` command for virtual contest participation ([4d2e594](https://github.com/t-seki/acr/commit/4d2e594555cd2c5fce07c2a6d27a26995a6aa741))
* add `acr virtual` command for virtual contest participation ([ebc533c](https://github.com/t-seki/acr/commit/ebc533c1373d14a0f10e4263a8911bb5b066090e)), closes [#47](https://github.com/t-seki/acr/issues/47)
* add `acrs view` command and improve sample case fetching ([941c599](https://github.com/t-seki/acr/commit/941c599d21bec5d8d4df8fe32bfdeeda924db0d6))
* add acrs view command and improve sample case fetching ([791aeb3](https://github.com/t-seki/acr/commit/791aeb3151b2fad15b3745d58c901a3be0a9f454))
* add contest info fetch and sample case scraping ([dcc1892](https://github.com/t-seki/acr/commit/dcc1892c77198736505308383e18ddc1e89fcbd3))
* add contest info fetch and sample case scraping ([3c5c6fa](https://github.com/t-seki/acr/commit/3c5c6faa2816f9da654c2e6bff2762c602273c80))
* add fetch command to re-fetch sample test cases ([3a77453](https://github.com/t-seki/acr/commit/3a77453992340e43c8221a9e55434a2aef9f18c9))
* add fetch command to re-fetch sample test cases ([f28e0ef](https://github.com/t-seki/acr/commit/f28e0ef6a87ebd4858609a8f03406a8c36c7a3c2))
* add itertools and std::collections to default template ([aab0430](https://github.com/t-seki/acr/commit/aab04300b1149cfc8328b321ee3d574d0e508e5c))
* add itertools and std::collections to default template ([2d69fef](https://github.com/t-seki/acr/commit/2d69fef6bfeea8d3eb5f82089c2e98fd5f7a7b32)), closes [#29](https://github.com/t-seki/acr/issues/29)
* add retry with backoff for test case fetching ([ed6bf3e](https://github.com/t-seki/acr/commit/ed6bf3ea9aa9c1ede37776f15b3692c470a1b068))
* add retry with backoff for test case fetching ([718a70f](https://github.com/t-seki/acr/commit/718a70f86f18b595586b3011b3d72b48a492faea))
* add single-letter aliases for new, view, test, submit ([4d7a084](https://github.com/t-seki/acr/commit/4d7a0844d41058f364d06ea54dede827cafe28ff))
* add single-letter aliases for new, view, test, submit commands ([61b6d07](https://github.com/t-seki/acr/commit/61b6d078cf69d07a39fbe6668edea25f1635d747))
* add submissions command ([2f6f76b](https://github.com/t-seki/acr/commit/2f6f76bc987b4ed977c71e0589f4396437ecb353))
* add submissions command to open submissions page in browser ([3f95835](https://github.com/t-seki/acr/commit/3f958355670ef05125b75f0102332c8922eb24fb))
* allow `acr view` from outside contest directory and normalize CLI args ([330decb](https://github.com/t-seki/acr/commit/330decb3c7226addb08c63973f8eba3500d1a71e))
* allow acr view from outside contest directory ([11b0311](https://github.com/t-seki/acr/commit/11b03116d066d01760e9b4da355a8d59e0e8967c))
* allow filtering problems on new and add all missing on add ([305fcb4](https://github.com/t-seki/acr/commit/305fcb44d4a43a3f4205dcb20c8e7cd69cf4e031))
* allow specifying problem for test/submit/view commands ([3e7e428](https://github.com/t-seki/acr/commit/3e7e4288f92cb4a5dd5c815173ee2c1c462179b6))
* allow specifying problem for test/submit/view commands ([8ce6b23](https://github.com/t-seki/acr/commit/8ce6b23d88519f9d89f39fb40f12736be43f38a9))
* allow submissions command to accept contest ID ([b62378a](https://github.com/t-seki/acr/commit/b62378a6e33d990914b2a8c2d8bb594540be399b))
* allow submissions command to accept contest ID argument ([8caba53](https://github.com/t-seki/acr/commit/8caba53861bc86d54dd7c020abea5a3bcba19751))
* fallback to tasks page when standings/json is unavailable ([e446b49](https://github.com/t-seki/acr/commit/e446b4990dbf6b123899392e4ada33a67186846f))
* fallback to tasks page when standings/json is unavailable ([944c329](https://github.com/t-seki/acr/commit/944c32959160d0a9c1c3a0801982f492cb9b54e8))
* generate .cargo/config.toml in acrs init for shared target dir ([97b5559](https://github.com/t-seki/acr/commit/97b555928eced17b1e54d2c1fadc7d6f35d8fdd7))
* generate .gitignore on acr init ([a1015bb](https://github.com/t-seki/acr/commit/a1015bbacb8599fa250fa6c0773abd11d14b97de))
* generate .gitignore on acr init ([8aa17b6](https://github.com/t-seki/acr/commit/8aa17b63f820ad516395a1ae236fb7bbf03f16a2))
* implement acrs new and acrs add commands ([4c29c6e](https://github.com/t-seki/acr/commit/4c29c6e6cb355e57b9960c4ef1e7a57e1039bb79))
* implement acrs new and acrs add commands ([955d261](https://github.com/t-seki/acr/commit/955d2615729091821cc9f38d8b06a30d076342ec))
* implement AtCoder auth ([68d5687](https://github.com/t-seki/acr/commit/68d568710f73dece8f513d4856d1748eaa11a2c7))
* implement AtCoder auth with scraper and login/logout/session commands ([d406955](https://github.com/t-seki/acr/commit/d406955abf9365acc6d40efcc98d5d82464cc990))
* implement config module with GlobalConfig and SessionConfig ([65b977c](https://github.com/t-seki/acr/commit/65b977cab70f86e61ce47d87fabf107e7effea9a))
* implement init and config commands ([7fb3bcb](https://github.com/t-seki/acr/commit/7fb3bcb1e881f2d4a051ac9a53dd14aaf29dcc46))
* implement init and config commands ([dcfccb4](https://github.com/t-seki/acr/commit/dcfccb46aad0091963c4fe9dfc258f19021d6b4a))
* implement submit command with result polling ([f2ae63c](https://github.com/t-seki/acr/commit/f2ae63cb49ab53c6ed73fdb6384b3c9d9dd41d61))
* implement submit command with result polling ([2f6c6ed](https://github.com/t-seki/acr/commit/2f6c6ed12381e6b133e979a944bd606447fd6365))
* implement test runner with build, execution, and colored output ([50d1325](https://github.com/t-seki/acr/commit/50d1325726cdf2bd2a0c9d16b0e9df6fc9797e5f))
* implement test runner with colored output ([76df6b3](https://github.com/t-seki/acr/commit/76df6b350cb180b3bcee81e8136fb7965c973c22))
* implement workspace generation and test case management ([643c0bc](https://github.com/t-seki/acr/commit/643c0bca83d96b7a057f833932c2072495dab438))
* implement workspace generation and test case management ([6435a9e](https://github.com/t-seki/acr/commit/6435a9e7c5b33668bf4ac3911768bc142e652d03))
* open first problem page in browser after contest creation ([eb72d57](https://github.com/t-seki/acr/commit/eb72d578f390ad0f0058b1568fe003611812779a))
* open first problem page in browser after contest creation ([22d34ca](https://github.com/t-seki/acr/commit/22d34cabf378c5492304f363bd8994c382211465))
* open first problem's source file in editor on contest creation ([ad6bd54](https://github.com/t-seki/acr/commit/ad6bd5446bffe3a8f7b17a2e2ea8d2d9283b04f8))
* parse browser config with shlex to support flags ([b8168a0](https://github.com/t-seki/acr/commit/b8168a06452600022ad32947097020fa211b368d)), closes [#64](https://github.com/t-seki/acr/issues/64)
* parse browser/editor config with shlex to support flags ([468c08f](https://github.com/t-seki/acr/commit/468c08ffc024816e1c66dca864c4c28dc68b164a))
* prepare for crates.io publishing ([af444af](https://github.com/t-seki/acr/commit/af444af3c7919a911f500735fa476dac7d662fa2))
* prepare for crates.io publishing ([15da42a](https://github.com/t-seki/acr/commit/15da42a0ae7a39b988f1e05830a110fae6b5c241))
* replace fetch command with update command ([b0fc7c5](https://github.com/t-seki/acr/commit/b0fc7c5798c7d5ca3d8f9b79b8834e4f99ac3901))
* replace fetch with update command ([b07609d](https://github.com/t-seki/acr/commit/b07609d673b09666bc7743214435a203450a20b6))
* replace login with browser cookie paste flow ([3f6fbbe](https://github.com/t-seki/acr/commit/3f6fbbe3eacaed3bbac3f86adae7cf90368e2dbc))
* replace login with browser cookie paste flow ([20c431f](https://github.com/t-seki/acr/commit/20c431fae7f027eb770e1b69e6e090e7ddd13d25))
* scaffold project with CLI definition and module structure ([8b4ca39](https://github.com/t-seki/acr/commit/8b4ca39a8b8ecf8c93fe8dad09de33b1fc8467d0))
* scaffold project with CLI definition and module structure ([7fa7dc6](https://github.com/t-seki/acr/commit/7fa7dc6f57263c87df081d9419c367f14de85f3f))
* shared target directory across contests ([995a4d7](https://github.com/t-seki/acr/commit/995a4d79187bc0e32da8e690a131d8173c50216d))
* switch submit to browser-based flow ([2059751](https://github.com/t-seki/acr/commit/20597512708cd84a9d00f49ae7670d8eb9ca8b39))
* switch submit to browser-based flow due to Cloudflare Turnstile ([9e1e8da](https://github.com/t-seki/acr/commit/9e1e8da1ecd51e7f7ceba3c8068f1e3d3167c03d))
* update problem dependencies to AtCoder 2025/10 library list ([75599b3](https://github.com/t-seki/acr/commit/75599b34f7e7945b32cfc7d30ca89c1be697aad9))
* update problem dependencies to AtCoder 2025/10 library list ([b7b475b](https://github.com/t-seki/acr/commit/b7b475ba8c2f3a45dd8101032d5d3e682e45d61f))
* use arboard for clipboard support ([c8e7202](https://github.com/t-seki/acr/commit/c8e7202398abcb88c0dfa53466cfb404c4c32291))
* use arboard for clipboard support ([3239df9](https://github.com/t-seki/acr/commit/3239df915b07d7a4bd7e6e11d11900e64d2b0805))
* 問題フィルタ指定と未追加問題の一括追加に対応 ([a78fecb](https://github.com/t-seki/acr/commit/a78fecbd03074b26e8c9e756d9c2427d20e14b42))


### Bug Fixes

* add initial delay and reduce first retry wait in --at mode ([a5de3a0](https://github.com/t-seki/acr/commit/a5de3a00b2653f0ca09ee21946c909fb5c4912fa))
* add initial delay and reduce first retry wait in --at mode ([6f610ae](https://github.com/t-seki/acr/commit/6f610ae477302851fd723ab813b512d62fddd045)), closes [#48](https://github.com/t-seki/acr/issues/48)
* avoid duplicate sample cases from lang-ja and lang-en sections ([12e1d9a](https://github.com/t-seki/acr/commit/12e1d9aa097d6d732a7750a2710f50c7ed202757))
* exclude format description from sample case extraction ([8bb9111](https://github.com/t-seki/acr/commit/8bb9111007ed7066a12415b72089bc1b669a93b3))
* exclude format description from sample case extraction ([0975071](https://github.com/t-seki/acr/commit/097507158c8ded70e11764f086730933aaf8074c))
* extract username from href instead of link text ([7bb8313](https://github.com/t-seki/acr/commit/7bb831327ae79ea3fc71a7aac9b5f22c628b7ab5))
* kill child process on TLE to prevent terminal hang ([3fb7b6e](https://github.com/t-seki/acr/commit/3fb7b6e6235c3b546169df8d4e6012f13e4baf31))
* kill child process on TLE to prevent terminal hang ([6688951](https://github.com/t-seki/acr/commit/6688951e2a833400b6e7df209adbbda855cf45fc)), closes [#58](https://github.com/t-seki/acr/issues/58)
* match generated Cargo.toml with AtCoder judge environment ([5dba839](https://github.com/t-seki/acr/commit/5dba8397a424889353f8850216c943095e8b69cb))
* match generated Cargo.toml with AtCoder judge environment ([25bf34b](https://github.com/t-seki/acr/commit/25bf34be103b39bcf464a9afd9b888dea1171ac8))
* progress bar missing newline and unify fetch messages ([315aa59](https://github.com/t-seki/acr/commit/315aa59d47eade7e3886e6298bbb4a04f32b9341))
* progress bar missing newline and unify test case fetch messages ([d9fe353](https://github.com/t-seki/acr/commit/d9fe353160afcfef86fa94b30555dd45b73c8006))
* retry on 404 errors in `--at` mode ([f8095f6](https://github.com/t-seki/acr/commit/f8095f6971d53048db463879b5f2ebcdad34f4a7))
* retry on 404 errors in `--at` mode ([c921820](https://github.com/t-seki/acr/commit/c921820d0efad2e09a3ef6b392d6235fdfaf1e2c)), closes [#48](https://github.com/t-seki/acr/issues/48)
* use valid crates.io keywords (max 20 chars each) ([eb5c71c](https://github.com/t-seki/acr/commit/eb5c71cd49211f0354f967455ec7718ce4c87759))
* warn on malformed browser/editor config instead of silent fallback ([6fa59e6](https://github.com/t-seki/acr/commit/6fa59e6f8d914a1dc65cdedb7a0b278c25ecb957))
