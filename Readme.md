# Awesome Systematic Trading

> or Quantitative Trading + a bit data science infra

[![Awesome](https://awesome.re/badge.svg)](https://awesome.re)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/wangzhe3224/awesome-systematic-trading/master)

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Java](https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=java&logoColor=white)
![C++](https://img.shields.io/badge/c++-%2300599C.svg?style=for-the-badge&logo=c%2B%2B&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Jupyter Notebook](https://img.shields.io/badge/jupyter-%23FA0F00.svg?style=for-the-badge&logo=jupyter&logoColor=white)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=wangzhe3224/awesome-systematic-trading&type=Timeline)](https://star-history.com/#wangzhe3224/awesome-systematic-trading&Timeline)

[希望阅读中文版？点我](./Readme_cn.md)

[Interested in systematic trading? Check QuantBox](https://quant.funcoder.net/)

A curated list of awesome libraries, packages and resources for Systematic Trading (Quantitative Trading)

> Open access: all rights granted for use and re-use of any kind, by anyone, at no cost, under your choice of either the free MIT License or Creative Commons CC-BY International Public License.

How do we pick the projects?

- Fit in Systematic Trading / Quantitative Trading domain
- Good coding style and software architecture
- (Optional) Under active development
- (Optional) Reasonable test coverage

Overall, I tend to pick decent or promising libraries that closely related to systematic trading instead of including as many libraries as possible.

 **Please raise a PR if you found some good fit projects for this repo or remove some outdated projects. Thanks!**

Search page by languages you are interested in to find related libraries. For example: `Ctrl+F`, `Rust`

And I count crypto as whole new category: [>> Click ME to Systematic Crypto](crypto_focus.md).

- [Awesome Systematic Trading](#awesome-systematic-trading)
  - [Star History](#star-history)
  - [🔥 AI Powered Systematic Trading Systems](#-ai-powered-systematic-trading-systems)
  - [Backtest + live trading](#backtest--live-trading)
    - [General purpose](#general-purpose)
    - [Crypto currency focus](#crypto-currency-focus)
    - [Machine Learning / Reinforcement Learning Focused](#machine-learning--reinforcement-learning-focused)
  - [Alpha Collections](#alpha-collections)
    - [General Alpha](#general-alpha)
    - [Expression based alpha](#expression-based-alpha)
    - [Stock picking](#stock-picking)
    - [Orderbook](#orderbook)
    - [Arbitrage (Crypto)](#arbitrage-crypto)
  - [Basic Components](#basic-components)
    - [Fundamental libraries](#fundamental-libraries)
    - [Computation](#computation)
    - [Python Performance Booster](#python-performance-booster)
    - [Python Profilers](#python-profilers)
    - [Alternative libraries](#alternative-libraries)
      - [Numpy Alternatives](#numpy-alternatives)
      - [Pandas Alternatives](#pandas-alternatives)
  - [Analytic tools](#analytic-tools)
    - [Metrics computation](#metrics-computation)
    - [Indicators](#indicators)
    - [Pricing](#pricing)
    - [Risk](#risk)
    - [Optimization](#optimization)
    - [TimeSeries Analysis](#timeseries-analysis)
  - [Visualization](#visualization)
  - [Message Queues](#message-queues)
  - [Databases](#databases)
  - [Data Source](#data-source)
    - [Stocks and General](#stocks-and-general)
    - [Alternative](#alternative)
    - [Crypto](#crypto)
  - [Broker APIs](#broker-apis)
  - [Quant Shops Code and Blog](#quant-shops-code-and-blog)
  - [Resources](#resources)
    - [Research](#research)
    - [Books](#books)
    - [Blogs](#blogs)
    - [Tutorials](#tutorials)
    - [Courses](#courses)
  - [Relevant Projects](#relevant-projects)

## 🔥 AI Powered Systematic Trading Systems

- [AI Hedge Fund](https://github.com/virattt/ai-hedge-fund) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/virattt/ai-hedge-fund/main) ![GitHub Repo stars](https://img.shields.io/github/stars/virattt/ai-hedge-fund?style=social) | `Python` | - An AI Hedge Fund Team
- [FinRL](https://github.com/AI4Finance-Foundation/FinRL) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/AI4Finance-Foundation/FinRL/master) ![GitHub Repo stars](https://img.shields.io/github/stars/AI4Finance-Foundation/FinRL?style=social) | Python | - FinRL is the first open-source framework to demonstrate the great potential of applying deep reinforcement learning in quantitative finance.
- [FinGPT](https://github.com/AI4Finance-Foundation/FinGPT) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/AI4Finance-Foundation/FinGPT/master) ![GitHub Repo stars](https://img.shields.io/github/stars/AI4Finance-Foundation/FinGPT?style=social) - FinGPT: Open-Source Financial Large Language Models! Revolutionize 🔥 We release the trained model on HuggingFace.
- [QLib (Microsoft)](https://github.com/microsoft/qlib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/microsoft/qlib/main) ![GitHub Repo stars](https://img.shields.io/github/stars/microsoft/qlib?style=social) | Python, Cython | - Qlib is an AI-oriented quantitative investment platform, which aims to realize the potential, empower the research, and create the value of AI technologies in quantitative investment. With Qlib, you can easily try your ideas to create better Quant investment strategies. An increasing number of SOTA Quant research works/papers are released in Qlib.
- [Qbot](https://github.com/UFund-Me/Qbot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/UFund-Me/Qbot/main) ![GitHub Repo stars](https://img.shields.io/github/stars/UFund-Me/Qbot?style=social) | `Python` | - AI 自动量化交易机器人 AI-powered Quantitative Investment Research Platform.

## Backtest + live trading

### General purpose

> Event Driven Frameworks

Note: the one marked as `Live Trading` has reasonable live trading support for at least 1 broker. Otherwise, backtest
 function only.

- [aat](https://github.com/AsyncAlgoTrading/aat) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/AsyncAlgoTrading/aat/main) ![GitHub Repo stars](https://img.shields.io/github/stars/AsyncAlgoTrading/aat?style=social) | Python, C++, Live Trading| - an asynchronous, event-driven framework for writing algorithmic trading strategies in python with optional acceleration in C++. It is designed to be modular and extensible, with support for a wide variety of instruments and strategies, live trading across (and between) multiple exchanges.
- [* barter-rs](https://github.com/barter-rs/barter-rs) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/barter-rs/barter-rs/main) ![GitHub Repo stars](https://img.shields.io/github/stars/barter-rs/barter-rs?style=social) | Rust | - Open-source Rust framework for building event-driven live-trading & backtesting systems. Algorithmic trade with the peace of mind that comes from knowing your strategies have been backtested with a near-identical trading Engine.
- [* bt](https://github.com/pmorissette/bt) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pmorissette/bt/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pmorissette/bt?style=social) | Python | -  Flexible backtesting for Python based on Algo and Strategy Tree
- [Better Quant](https://github.com/byrnexu/betterquant) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/byrnexu/betterquant/master) ![GitHub Repo stars](https://img.shields.io/github/stars/byrnexu/betterquant?style=social) | C++, Live Trading | - Better quant today, best quant tomorrow. 💪
- [Botvana](https://github.com/featherenvy/botvana) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/featherenvy/botvana/main) ![GitHub Repo stars](https://img.shields.io/github/stars/featherenvy/botvana?style=social) | Rust | - high-performance and event-driven trading system built using Rust
- [backtrader](https://github.com/mementum/backtrader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/mementum/backtrader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/mementum/backtrader?style=social) | Python, Live Trading | - Event driven Python Backtesting library for trading strategies
- [backtesting.py](https://github.com/kernc/backtesting.py) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kernc/backtesting.py/master) ![GitHub Repo stars](https://img.shields.io/github/stars/kernc/backtesting.py?style=social) | Python | - Backtesting.py is a Python framework for inferring viability of trading strategies on historical (past) data. Improved upon the vision of Backtrader, and by all means surpassingly comparable to other accessible alternatives, Backtesting.py is lightweight, fast, user-friendly, intuitive, interactive, intelligent and, hopefully, future-proof.
- [FlashFunk](https://github.com/HFQR/FlashFunk) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/HFQR/FlashFunk/master) ![GitHub Repo stars](https://img.shields.io/github/stars/HFQR/FlashFunk?style=social) | Rust | -  High Performance Runtime in Rust
- [QuantFabric](https://github.com/QuantFabric/QuantFabric) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/QuantFabric/QuantFabric/master) ![GitHub Repo stars](https://img.shields.io/github/stars/QuantFabric/QuantFabric?style=social) | C++ | - QuantFabric是基于Linux/C++开发的中高频量化交易系统，支持中金所、郑商所、大商所、上期所、上海国际能源中心的期货业务品种交易，支持上交所、深交所的股票、债券品种交易。
- [gobacktest](https://github.com/gobacktest/gobacktest) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/gobacktest/gobacktest/main) ![GitHub Repo stars](https://img.shields.io/github/stars/gobacktest/gobacktest?style=social) | Go | - A Go implementation of event-driven backtesting framework
- [Hikyuu](https://github.com/fasiondog/hikyuu) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/fasiondog/hikyuu/master) ![GitHub Repo stars](https://img.shields.io/github/stars/fasiondog/hikyuu?style=social) | C++, Python| - Hikyuu Quant Framework 基于C++/Python的开源量化交易研究框架
- [Investing Algorithm Framework](https://github.com/coding-kitties/investing-algorithm-framework/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/coding-kitties/investing-algorithm-framework/main) ![GitHub Repo stars](https://img.shields.io/github/stars/coding-kitties/investing-algorithm-framework?style=social) | Python | - Framework for developing, backtesting, and deploying automated trading algorithms and trading bots.
- [lumibot](https://github.com/Lumiwealth/lumibot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Lumiwealth/lumibot/master) ![GitHub Repo stars](https://img.shields.io/github/stars/Lumiwealth/lumibot?style=social) | Python | - A very simple yet useful backtesting and sample based live trading framework (a bit slow to run...)
- [* nautilus_trader](https://github.com/nautechsystems/nautilus_trader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nautechsystems/nautilus_trader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/nautechsystems/nautilus_trader?style=social) | Python, Cython, Rust, Live Trading | - A high-performance algorithmic trading platform and event-driven backtester
- [PyBroker](https://github.com/edtechre/pybroker) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/edtechre/pybroker/master) ![GitHub Repo stars](https://img.shields.io/github/stars/edtechre/pybroker?style=social) | Python | - Algorithmic Trading in Python with Machine Learning
- [QuantConnect](https://github.com/QuantConnect/Lean) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/QuantConnect/Lean/master) ![GitHub Repo stars](https://img.shields.io/github/stars/QuantConnect/Lean?style=social) | C#, .NET, Live Trading | - Lean Algorithmic Trading Engine by QuantConnect (Python, C#)
- [QUANTAXIS](https://github.com/QUANTAXIS/QUANTAXIS) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/QUANTAXIS/QUANTAXIS/master) ![GitHub Repo stars](https://img.shields.io/github/stars/QUANTAXIS/QUANTAXIS?style=social) | Python, Rust, Live Trading | - QUANTAXIS 支持任务调度 分布式部署的 股票/期货/期权/港股/虚拟货币 数据/回测/模拟/交易/可视化/多账户 纯本地量化解决方案
- [Rqalpha](https://github.com/ricequant/rqalpha) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/ricequant/rqalpha/master) ![GitHub Repo stars](https://img.shields.io/github/stars/ricequant/rqalpha?style=social) | Python | - A extendable, replaceable Python algorithmic backtest && trading framework supporting multiple securities
- [quanttrader](https://github.com/letianzj/quanttrader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/letianzj/quanttrader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/letianzj/quanttrader?style=social) | Python | - Backtest and live trading in Python. Event based. Similar to backtesting.py.
- [qf-lib](https://github.com/quarkfin/qf-lib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/quarkfin/qf-lib/master) ![GitHub Repo stars](https://img.shields.io/github/stars/quarkfin/qf-lib?style=social) | Python | - Modular Python library that provides an advanced event driven backtester and a set of high quality tools for quantitative finance. Integrated with various data vendors and brokers, supports Crypto, Stocks and Futures.
- [sdoosa-algo-trade-python](https://github.com/sreenivasdoosa/sdoosa-algo-trade-python) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/sreenivasdoosa/sdoosa-algo-trade-python/master) ![GitHub Repo stars](https://img.shields.io/github/stars/sreenivasdoosa/sdoosa-algo-trade-python?style=social) | Python | - This project is mainly for newbies into algo trading who are interested in learning to code their own trading algo using python interpreter.
- [* vnpy](https://github.com/vnpy/vnpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/vnpy/vnpy/master) ![GitHub Repo stars](https://img.shields.io/github/stars/vnpy/vnpy?style=social) | Python, Stock, Futures, Crypto, Live Trading | - Python-based open source quantitative trading system development framework, officially released in January 2015, has grown step by step into a full-featured quantitative trading platform
- [WonderTrader](https://github.com/wondertrader/wondertrader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/wondertrader/wondertrader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/wondertrader/wondertrader?style=social) | C++, Python | - WonderTrader——量化研发交易一站式框架
- [zvt](https://github.com/zvtvz/zvt) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/zvtvz/zvt/master) ![GitHub Repo stars](https://img.shields.io/github/stars/zvtvz/zvt?style=social) | Python, Stock, Backtest | - Modular quant framework
- [zipline](https://github.com/quantopian/zipline) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/quantopian/zipline/master) ![GitHub Repo stars](https://img.shields.io/github/stars/quantopian/zipline?style=social) | Python | - Zipline is a Pythonic algorithmic trading library. It is an event-driven system for backtesting.
- [PandoraTrader](https://github.com/pegasusTrader/PandoraTrader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pegasusTrader/PandoraTrader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pegasusTrader/PandoraTrader?style=social) | C++ | - CTP 高频量化交易平台 C++ Trade Platform for quant developer
- [hftbacktest](https://github.com/nkaz001/hftbacktest) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nkaz001/hftbacktest/master) ![GitHub Repo stars](https://img.shields.io/github/stars/nkaz001/hftbacktest?style=social) | Python, numba | - A high-frequency trading and market-making backtesting tool accounts for limit orders, queue positions, and latencies, utilizing full tick data for trades and order books.
- [Cipher](https://github.com/nanvel/cipher-bt) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nanvel/cipher-bt/master) ![GitHub Repo stars](https://img.shields.io/github/stars/nanvel/cipher-bt?style=social) | Python | - Backtesting library with focus on position adjustment that allows testing complicated setups. Pythonic, extensible, well-structured, documented.
- [Gunbot Quant](https://github.com/GuntharDeNiro/gunbot-quant) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/GuntharDeNiro/gunbot-quant/master) ![GitHub Repo stars](https://img.shields.io/github/stars/GuntharDeNiro/gunbot-quant?style=social) | Python | - Standalone application for market screening and backtesting. Supports multi-asset, multi-strategy backtests. Includes user interface and CLI options.

> Vector Based Frameworks

- [QTradeX](https://github.com/squidKid-deluxe/QTradeX-Algo-Trading-SDK)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/squidKid-deluxe/QTradeX-Algo-Trading-SDK) ![GitHub Repo stars](https://img.shields.io/github/stars/squidKid-deluxe/QTradeX-Algo-Trading-SDK?style=social) | Python, Live Trading | - A powerful and flexible Python framework for designing, backtesting, optimizing, and deploying algotrading bots
- [FinHack](https://github.com/FinHackCN/finhack) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/FinHackCN/finhack/main) ![GitHub Repo stars](https://img.shields.io/github/stars/FinHackCN/finhack?style=social) | Python | - 一个易于拓展的量化金融框架，它在当前版本中集成了数据采集、因子计算、因子挖掘、因子分析、机器学习、策略编写、量化回测、实盘接入等全流程的量化投研工作
- [pysystemtrade](https://github.com/robcarver17/pysystemtrade) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/robcarver17/pysystemtrade/master) ![GitHub Repo stars](https://img.shields.io/github/stars/robcarver17/pysystemtrade?style=social) | Python, Live Trading | - Systematic Trading in python from book <Systematic Trading> by Rob Carver
- [finmarketpy](https://github.com/cuemacro/finmarketpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cuemacro/finmarketpy/master) ![GitHub Repo stars](https://img.shields.io/github/stars/cuemacro/finmarketpy?style=social) | Python | - Python library for backtesting trading strategies & analyzing financial markets (formerly pythalesians)
- [vectorbt](https://github.com/polakowo/vectorbt) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/polakowo/vectorbt/master) ![GitHub Repo stars](https://img.shields.io/github/stars/polakowo/vectorbt?style=social) | Python, numba | - vectorbt takes a novel approach to backtesting: it operates entirely on pandas and NumPy objects, and is accelerated by Numba to analyze any data at speed and scale. This allows for testing of many thousands of strategies in seconds.
- [fund-strategy](https://github.com/SunshowerC/fund-strategy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/SunshowerC/fund-strategy/master) ![GitHub Repo stars](https://img.shields.io/github/stars/SunshowerC/fund-strategy?style=social) | TypeScript | - 一个简单实用的基金投资策略分析，基金回测工具
- [fastquant](https://github.com/enzoampil/fastquant) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/enzoampil/fastquant/master) ![GitHub Repo stars](https://img.shields.io/github/stars/enzoampil/fastquant?style=social) | Python | - Backtest and optimize your ML trading strategies with only 3 lines of code

### Crypto currency focus

- [basana](https://github.com/gbeced/basana) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/gbeced/basana/master) ![GitHub Repo stars](https://img.shields.io/github/stars/gbeced/basana?style=social) | Python | - A Python async and event driven framework for algorithmic trading, with a focus on crypto currencies.
- [c-binance-future-quant](https://github.com/Melelery/c-binance-future-quant/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Melelery/c-binance-future-quant/main) ![GitHub Repo stars](https://img.shields.io/github/stars/Melelery/c-binance-future-quant?style=social) | Python | - 低成本，高效率，简单实现的币安合约量化系统架构
- [triangular-arbitrage2](https://github.com/zlq4863947/triangular-arbitrage2) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/zlq4863947/triangular-arbitrage2/main) ![GitHub Repo stars](https://img.shields.io/github/stars/zlq4863947/triangular-arbitrage2?style=social) | TypeScript | - a server side application for perform triangular arbitrage.
- [bTrader](https://github.com/gabriel-milan/btrader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/gabriel-milan/btrader/master) ![GitHub Repo stars](https://img.shields.io/github/stars/gabriel-milan/btrader?style=social) | Rust | - Triangle arbitrage trading bot for Binance
- [crypto-crawler-rs](https://github.com/crypto-crawler/crypto-crawler-rs) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/crypto-crawler/crypto-crawler-rs/main) ![GitHub Repo stars](https://img.shields.io/github/stars/crypto-crawler/crypto-crawler-rs?style=social) | Rust | - Crawl orderbook and trade messages from crypto exchanges
- [cryptotrader-core](https://github.com/monomadic/cryptotrader-core) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/monomadic/cryptotrader-core/master) ![GitHub Repo stars](https://img.shields.io/github/stars/monomadic/cryptotrader-core?style=social) | Rust | - Simple to use Crypto Exchange REST API client in rust.
- [openlimits](https://github.com/nash-io/openlimits) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nash-io/openlimits/main) ![GitHub Repo stars](https://img.shields.io/github/stars/nash-io/openlimits?style=social) | Rust | - A Rust high performance cryptocurrency trading API with support for multiple exchanges and language wrappers.
- [Freqtrade](https://github.com/freqtrade/freqtrade) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/freqtrade/freqtrade/develop) ![GitHub Repo stars](https://img.shields.io/github/stars/freqtrade/freqtrade?style=social) | Python | - Freqtrade is a free and open source crypto trading bot written in Python. It is designed to support all major exchanges and be controlled via Telegram. It contains backtesting, plotting and money management tools as well as strategy optimization by machine learning.
- [* Hummingbot](https://github.com/CoinAlpha/hummingbot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/CoinAlpha/hummingbot/development) ![GitHub Repo stars](https://img.shields.io/github/stars/CoinAlpha/hummingbot?style=social) | Python, Cython, Live Trading | - A client for crypto market making
- [Jesse](https://github.com/jesse-ai/jesse) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/jesse-ai/jesse/master) ![GitHub Repo stars](https://img.shields.io/github/stars/jesse-ai/jesse?style=social) | Python | - Jesse is an advanced crypto trading framework which aims to simplify researching and defining trading strategies.
- [* OctoBot](https://github.com/Drakkar-Software/OctoBot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Drakkar-Software/OctoBot/master) ![GitHub Repo stars](https://img.shields.io/github/stars/Drakkar-Software/OctoBot?style=social) | Python, Cython, Live Trading| - Cryptocurrency trading bot for TA, arbitrage and social trading with an advanced web interface
- [Kelp](https://github.com/stellar/kelp) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/stellar/kelp/master) ![GitHub Repo stars](https://img.shields.io/github/stars/stellar/kelp?style=social) | Go, Live Trading | - Kelp is a free and open-source trading bot for the Stellar DEX and 100+ centralized exchanges
- [exc](https://github.com/Nouzan/exc) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Nouzan/exc/main) ![GitHub Repo stars](https://img.shields.io/github/stars/Nouzan/exc?style=social) | Rust | - The abstraction layer of exchanges.
- [MyCryptoBot](https://github.com/diogomatoschaves/MyCryptoBot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/diogomatoschaves/MyCryptoBot/master) ![GitHub Repo stars](https://img.shields.io/github/stars/diogomatoschaves/MyCryptoBot?style=social) | Python, Js | - Automated, open source crypto trading and backtesting platform

### Machine Learning / Reinforcement Learning Focused

> ML, RL

- [TradingGym](https://github.com/Yvictor/TradingGym) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Yvictor/TradingGym/master) ![GitHub Repo stars](https://img.shields.io/github/stars/Yvictor/TradingGym?style=social) | Python, Live Trading | - Trading and Backtesting environment for training reinforcement learning agent or simple rule base algo.
- [Stock Trading Bot using Deep Q-Learning](https://github.com/pskrunner14/trading-bot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pskrunner14/trading-bot/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pskrunner14/trading-bot?style=social) | Python | - Stock Trading Bot using Deep Q-Learning

## Alpha Collections

### General Alpha

- [Python quantitative trading strategies including VIX Calculator, Pattern Recognition, Commodity Trading Advisor, Monte Carlo, Options Straddle, Shooting Star, London Breakout, Heikin-Ashi, Pair Trading, RSI, Bollinger Bands, Parabolic SAR, Dual Thrust, Awesome, MACD](https://github.com/je-suis-tm/quant-trading#15-vix-calculator)
- [analyzingalpha](https://github.com/leosmigel/analyzingalpha)
- [Finance](https://github.com/shashankvemuri/Finance) | `Python` | - 150+ quantitative finance Python programs to help you gather, manipulate, and analyze stock market data
- [ThetaGang](https://github.com/brndnmtthws/thetagang) - ThetaGang is an IBKR bot for collecting money
  - <https://www.reddit.com/r/options/comments/a36k4j/the_wheel_aka_triple_income_strategy_explained/>
- [PyTrendFollow](https://github.com/chrism2671/PyTrendFollow) | `Python` | - PyTrendFollow - systematic futures trading using trend following
- [czsc - 缠中说禅技术分析工具](https://github.com/waditu/czsc) | `Python` | - 缠中说禅技术分析工具；缠论；股票；期货；Quant；量化交易
- [volest](https://github.com/jasonstrimpel/volatility-trading) | `Python` | - A complete set of volatility estimators based on Euan Sinclair's Volatility Trading
- [quant-trading](https://github.com/je-suis-tm/quant-trading) | `Python` | - Python quantitative trading strategies including VIX Calculator, Pattern Recognition, Commodity Trading Advisor, Monte Carlo, Options Straddle, Shooting Star, London Breakout, Heikin-Ashi, Pair Trading, RSI, Bollinger Bands, Parabolic SAR, Dual Thrust, Awesome, MACD
- [一个中文策略合集](https://github.com/fmzquant/strategies) | `Python` |
- [一个实盘的股票趋势策略](https://github.com/BigBrotherTrade/trader) | `Python` | -
- [Quantitative-analysis](https://github.com/hugo2046/QuantsPlaybook) | `Python` | - 量化研究-券商金工研报复现

### Expression based alpha

- [torchquantum](https://github.com/nymath/torchquantum) | `Cython`, `C`, `Python` | - TorchQuantum is a backtesting framework that integrates the structure of PyTorch and WorldQuant's Operator for efficient quantitative financial analysis.
- [OpenAlpha](https://github.com/caoruicn/openalpha) | `C++` | - An open source equity statistical arbitrage backtest simulator, use the same API as WorldQuant's WebSim
- [stock](https://github.com/xcycharles/stock) | `Python` | - 一些因子挖掘的代码 A 股
- [AlphaGen](https://github.com/RL-MLDM/alphagen) | `Python` | - Automatic formulaic alpha generation with reinforcement learning.
- [Genetic-Alpha](https://github.com/Morgansy/Genetic-Alpha) - A genetic programming algorithm used for generating alpha factors in the multi-factor investment strategy
- [alpha_examples](https://github.com/wukan1986/alpha_examples) - An expression based alpha demo using Polars

### Stock picking

- [InvesTool](https://github.com/axiaoxin-com/investool) | `Go` | - Golang实现财报分析、个股基本面检测、基本面选股、4433法则基金筛选与检测、基金持仓相似度、股票选基、基金经理筛选
- [Sequoia选股系统](https://github.com/sngyai/Sequoia#sequoia%E9%80%89%E8%82%A1%E7%B3%BB%E7%BB%9F) | `Python` | - A股自动选股程序，实现了海龟交易法则、缠中说禅牛市买点，以及其他若干种技术形态

### Orderbook

- [The Microprice](https://github.com/sstoikov/microprice/tree/master) - An estimator of the fair price, given the state of the order book.

### Arbitrage (Crypto)

> Note: these bots are old and not maintained. I put them here just to show some logic of crypto arbitrage.
  
- [Blackbird](https://github.com/butor/blackbird) | `C++` | - Blackbird Bitcoin Arbitrage: a long/short market-neutral strategy
- [bitcoin-arbitrage](https://github.com/maxme/bitcoin-arbitrage) | `Python` | - Bitcoin arbitrage - opportunity detector
- [R2 Bitcoin Arbitrager](https://github.com/bitrinjani/r2) | `TypeScript` | - R2 Bitcoin Arbitrager is an automatic arbitrage trading system powered by Node.js + TypeScript.

## Basic Components

### Fundamental libraries

- [Cvxpy](https://github.com/cvxpy/cvxpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cvxpy/cvxpy/master) ![GitHub Repo stars](https://img.shields.io/github/stars/cvxpy/cvxpy?style=social) | Python, C++ | - A Python-embedded modeling language for convex optimization problems.
- [jax](https://github.com/google/jax) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/google/jax/main) ![GitHub Repo stars](https://img.shields.io/github/stars/google/jax?style=social) | `Python` | - Composable transformations of Python+NumPy programs: differentiate, vectorize, JIT to GPU/TPU, and more
- [Numpy](https://github.com/numpy/numpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numpy/numpy/main) ![GitHub Repo stars](https://img.shields.io/github/stars/numpy/numpy?style=social) | Python, C | - The fundamental package for scientific computing with Python
- [trade-frame](https://github.com/rburkholder/trade-frame) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/rburkholder/trade-frame/master) ![GitHub Repo stars](https://img.shields.io/github/stars/rburkholder/trade-frame?style=social) | C++ | - C++ 17 based library (with sample applications) for testing equities, futures, currencies, etfs & options based automated trading ideas using DTN IQFeed real time data feed and Interactive Brokers (IB TWS API) for trade execution. Some support for Alpaca & Phemex. Notifications via Telegram 
- Modelling
  - [Scipy](https://github.com/scipy/scipy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/scipy/scipy/main) ![GitHub Repo stars](https://img.shields.io/github/stars/scipy/scipy?style=social) | Python, C | - Fundamental algorithms for scientific computing in Python
  - [statsmodels](https://github.com/statsmodels/statsmodels/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/statsmodels/statsmodels/main) ![GitHub Repo stars](https://img.shields.io/github/stars/statsmodels/statsmodels?style=social) - Python module that allows users to explore data, estimate statistical models, and perform statistical tests.
  - [PyMC](https://github.com/pymc-devs/pymc) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/statsmodels/statsmodels/main) ![GitHub Repo stars](https://img.shields.io/github/stars/pymc-devs/pymc?style=social) | Python | - Probabilistic Programming in Python: Bayesian Modeling and Probabilistic Machine Learning with Aesara
  - [DEAP](https://github.com/DEAP/deap) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/DEAP/deap/master) ![GitHub Repo stars](https://img.shields.io/github/stars/DEAP/deap?style=social) |Python| - Distributed Evolutionary Algorithms in Python
- DataFrame
  - [Pandas](https://github.com/pandas-dev/pandas) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pandas-dev/pandas/main) ![GitHub Repo stars](https://img.shields.io/github/stars/pandas-dev/pandas?style=social) | Python, Cython | - Flexible and powerful data analysis / manipulation library for Python, providing labeled data structures similar to R data.frame objects, statistical functions, and much more
  - [Polars](https://github.com/pola-rs/polars) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pola-rs/polars/main) ![GitHub Repo stars](https://img.shields.io/github/stars/pola-rs/polars?style=social)| Rust, Python | - Polars is a blazingly fast DataFrames library implemented in Rust using Apache Arrow Columnar Format as memory model.
  - [FireDucks](https://fireducks-dev.github.io/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/fireducks-dev/fireducks/main) - Compiler Accelerated DataFrame Library for Python with fully-compatible pandas API
- Machine Learning
  - [Hugging Face](https://github.com/huggingface/) ![GitHub Repo stars](https://img.shields.io/github/stars/huggingface?style=social) - The AI community building the future.
  - [LangChain](https://github.com/langchain-ai/langchain) ![GitHub Repo stars](https://img.shields.io/github/stars/langchain-ai/langchain?style=social) - Building applications with LLMs through composability
  - [Sikit-learn](https://github.com/scikit-learn/scikit-learn) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/scikit-learn/scikit-learn/main) ![GitHub Repo stars](https://img.shields.io/github/stars/scikit-learn/scikit-learn?style=social) | Python, Cython | - Machine learning in Python
  - [JAX](https://github.com/google/jax) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/google/jax/main) ![GitHub Repo stars](https://img.shields.io/github/stars/google/jax?style=social)  - Composable transformations of Python+NumPy programs: differentiate, vectorize, JIT to GPU/TPU, and more
  - [Pytorch](https://github.com/pytorch/pytorch) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pytorch/pytorch/main) ![GitHub Repo stars](https://img.shields.io/github/stars/pytorch/pytorch?style=social) | Python | - Tensors and Dynamic neural networks in Python with strong GPU acceleration
  - [Keras](https://github.com/keras-team/keras) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/keras-team/keras/master) ![GitHub Repo stars](https://img.shields.io/github/stars/keras-team/keras?style=social) | Python | - The most user friendly Deep Learning for humans in Python
  - [TensorFlow](https://github.com/tensorflow/tensorflow) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/tensorflow/tensorflow/master) ![GitHub Repo stars](https://img.shields.io/github/stars/tensorflow/tensorflow?style=social) | Python, C++ | - More low level Deep Learning framework
- DAG
  - [Rustworkx](https://github.com/Qiskit/rustworkx/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Qiskit/rustworkx/main) ![GitHub Repo stars](https://img.shields.io/github/stars/Qiskit/rustworkx?style=social) | Rust, Python | - A high performance Python graph library implemented in Rust.
  - [Networkx](https://github.com/networkx/networkx) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/networkx/networkx/main) ![GitHub Repo stars](https://img.shields.io/github/stars/networkx/networkx?style=social) | Python | - Network Analysis in Python

### Computation

- [Ray](https://github.com/ray-project/ray) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/ray-project/ray/master) ![GitHub Repo stars](https://img.shields.io/github/stars/ray-project/ray?style=social) | Python, C++ | - An open source framework that provides a simple, universal API for building distributed applications.
- [csp (Point72)](https://github.com/Point72/csp/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Point72/csp/main) ![GitHub Repo stars](https://img.shields.io/github/stars/Point72/csp?style=social) | Python, C++ | - csp is a high performance reactive stream processing library, written in C++ and Python
- [Dask](https://github.com/dask/dask) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dask/dask/main) ![GitHub Repo stars](https://img.shields.io/github/stars/dask/dask?style=social) | Python | - Parallel computing with task scheduling in Python with a Pandas like API
- [Spark](https://github.com/apache/spark) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/apache/spark/master) ![GitHub Repo stars](https://img.shields.io/github/stars/apache/spark?style=social) | Scala | - Apache Spark - A unified analytics engine for large-scale data processing
- [Hamilton](https://github.com/dagworks-inc/hamilton) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dagworks-inc/hamilton/main) ![GitHub Repo stars](https://img.shields.io/github/stars/dagworks-inc/hamilton?style=social)| Python | - A scalable general purpose micro-framework for defining dataflows. You can use it to build dataframes, numpy matrices, python objects, ML models, etc. Embed Hamilton anywhere python runs, e.g. spark, airflow, jupyter, fastapi, python scripts, etc.
- [Incremental (JaneStreet)](https://github.com/janestreet/incremental) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/janestreet/incremental/master) ![GitHub Repo stars](https://img.shields.io/github/stars/janestreet/incremental?style=social) | Ocaml | - Incremental is a library that gives you a way of building complex computations that can update efficiently in response to their inputs changing, inspired by the work of Umut Acar et. al. on self-adjusting computations. Incremental can be useful in a number of applications
- [Joblib](https://github.com/joblib/joblib)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/joblib/joblib/main) ![GitHub Repo stars](https://img.shields.io/github/stars/joblib/joblib?style=social) | Python | - running Python functions as pipeline jobs
- [Tributary](https://github.com/timkpaine/tributary) ![GitHub Repo stars](https://img.shields.io/github/stars/timkpaine/tributary?style=social) | Python | - Streaming reactive and dataflow graphs in Python
- [GraphKit(No activity)](https://github.com/yahoo/graphkit) ![GitHub Repo stars](https://img.shields.io/github/stars/yahoo/graphkit?style=social) | Python | - A lightweight Python module for creating and running ordered graphs of computations.
- [Man MDF (No activity)](https://github.com/man-group/mdf) ![GitHub Repo stars](https://img.shields.io/github/stars/man-group/mdf?style=social) | Python | - Data-flow programming toolkit for Python
- [Anchors - C++(No activity)](https://github.com/oluwatimilehin/anchors) ![GitHub Repo stars](https://img.shields.io/github/stars/oluwatimilehin/anchors?style=social) | C++ | - C++ library for incremental computing
- [Anchors - Rust(No activity)](https://github.com/lord/anchors) ![GitHub Repo stars](https://img.shields.io/github/stars/lord/anchors?style=social) | Rust | - self adjusting computations in rust
- [Loman (No activity)](https://github.com/janushendersonassetallocation/loman) ![GitHub Repo stars](https://img.shields.io/github/stars/janushendersonassetallocation/loman?style=social) | Python | - Loman is a Python library designed to allow quantitative researchers to control complex live updating calculation processes

### Python Performance Booster

- [cython](https://github.com/cython/cython) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cython/cython/master) ![GitHub Repo stars](https://img.shields.io/github/stars/cython/cython?style=social) - Cython is a Python compiler that makes writing C extensions for Python as easy as Python itself. Cython is based on Pyrex, but supports more cutting edge functionality and optimizations.
- [numba](https://github.com/numba/numba) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numba/numba/main) ![GitHub Repo stars](https://img.shields.io/github/stars/numba/numba?style=social) - NumPy aware dynamic Python compiler using LLVM
- [pybind11](https://github.com/pybind/pybind11) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pybind/pybind11/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pybind/pybind11?style=social) - Seamless operability between C++11 and Python
- [pyo3](https://github.com/PyO3/pyo3)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numba/numba/main) ![GitHub Repo stars](https://img.shields.io/github/stars/PyO3/pyo3?style=social) - Rust bindings for the Python interpreter
- [CuPy](https://github.com/cupy/cupy/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cupy/cupy/main) ![GitHub Repo stars](https://img.shields.io/github/stars/cupy/cupy?style=social) | Python, C++, Cython, Cuda | - CuPy is an open-source array library for GPU-accelerated computing with Python. 100x Boost for some operations
- [CuDF](https://github.com/rapidsai/cudf) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/rapidsai/cudf/main) ![GitHub Repo stars](https://img.shields.io/github/stars/rapidsai/cudf?style=social) | Python | - cuDF - GPU DataFrame Library. No-code-change accelerator for pandas.
- [codon](https://github.com/exaloop/codon) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/exaloop/codon/develop) ![GitHub Repo stars](https://img.shields.io/github/stars/exaloop/codon?style=social) | C++ | - A high-performance, zero-overhead, extensible Python compiler using LLVM
- [Bottleneck](https://github.com/pydata/bottleneck) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pydata/bottleneck/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pydata/bottleneck?style=social) | Python, C | - Fast NumPy array functions written in C
- [NumExpr](https://github.com/pydata/numexpr) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pydata/numexpr/master) ![GitHub Repo stars](https://img.shields.io/github/stars/pydata/numexpr?style=social) | Python, C++ | - Fast numerical array expression evaluator for Python, NumPy, PyTables, pandas, bcolz and more
- [pandarallel](https://github.com/nalepae/pandarallel) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nalepae/pandarallel/master) ![GitHub Repo stars](https://img.shields.io/github/stars/nalepae/pandarallel?style=social) | Python | - A simple and efficient tool to parallelize Pandas operations on all available CPUs

### Python Profilers

- [py-spy](https://github.com/benfred/py-spy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/benfred/py-spy/master) ![GitHub Repo stars](https://img.shields.io/github/stars/benfred/py-spy?style=social) - Sampling profiler for Python programs
- [pyinstrument](https://github.com/joerick/pyinstrument) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/joerick/pyinstrument/main) ![GitHub Repo stars](https://img.shields.io/github/stars/joerick/pyinstrument?style=social) - Call stack profiler for Python. Shows you why your code is slow!
- [Memray](https://github.com/bloomberg/memray) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/bloomberg/memray/main) ![GitHub Repo stars](https://img.shields.io/github/stars/bloomberg/memray?style=social) - Memray is a memory profiler for Python

### Alternative libraries

#### Numpy Alternatives

- [ndarray](https://github.com/rust-ndarray/ndarray) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/rust-ndarray/ndarray/master) ![GitHub Repo stars](https://img.shields.io/github/stars/rust-ndarray/ndarray?style=social) | Rust | - ndarray: an N-dimensional array with array views, multidimensional slicing, and efficient operations
- [faer](https://github.com/sarah-ek/faer-rs)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/sarah-ek/faer-rs/main) ![GitHub Repo stars](https://img.shields.io/github/stars/sarah-ek/faer-rs?style=social) | Rust | - Linear algebra foundation for the Rust programming language

#### Pandas Alternatives

- [DataFrame](https://github.com/hosseinmoein/DataFrame) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/hosseinmoein/DataFrame/master) ![GitHub Repo stars](https://img.shields.io/github/stars/hosseinmoein/DataFrame?style=social)  | C++ | - C++ DataFrame for statistical, Financial, and ML analysis -- in modern C++ using native types and contiguous memory storage
- [Vaex](https://github.com/vaexio/vaex) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/vaexio/vaex/master) ![GitHub Repo stars](https://img.shields.io/github/stars/vaexio/vaex?style=social) | Python, C++ | - Out-of-Core hybrid Apache Arrow/NumPy DataFrame for Python, ML, visualization and exploration of big tabular data at a billion rows per second
- [Modin](https://github.com/modin-project/modin) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/modin-project/modin/main) ![GitHub Repo stars](https://img.shields.io/github/stars/modin-project/modin?style=social) | Python |  - Modin: Speed up your Pandas workflows by changing a single line of code
- [Koalas](https://github.com/databricks/koalas) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/databricks/koalas/master) ![GitHub Repo stars](https://img.shields.io/github/stars/databricks/koalas?style=social) | Python | - Koalas: pandas API on Apache Spark

## Analytic tools

### Metrics computation

- [alphalens (Fork)](https://github.com/wangzhe3224/alphalens) | `Python` | - Performance analysis of predictive (alpha) stock factors
- [ffn](https://github.com/pmorissette/ffn) | `Python` | - A financial function library for Python
- [quantstats](https://github.com/ranaroussi/quantstats) | `Python` | - Portfolio analytics for quants, written in Python

### Indicators

- [TA-Lib](https://ta-lib.org) | `C` | - Perform technical analysis of financial market data
  - [Python Wrapper](https://github.com/mrjbq7/ta-lib) | `Python` |
  - [Go Port](https://github.com/markcheno/go-talib) | `Go` |
  - [Rust Wrapper](https://github.com/CLevasseur/ta-lib-rust) | `Rust` |
- [ta-rust](https://github.com/greyblake/ta-rs) | `Rust` | - Technical analysis library for Rust language
- [finta](https://github.com/peerchemist/finta) | `Python` | - Common financial technical indicators implemented in Pandas
- [pandas-ta](https://github.com/twopirllc/pandas-ta) | `Python` | - Pandas Technical Analysis (Pandas TA) is an easy to use library that leverages the Pandas package with more than 130 Indicators and Utility functions and more than 60 TA Lib Candlestick Patterns.
- [kand](https://github.com/rust-ta/kand) | `Rust` & `Python` | - A blazingly fast technical analysis library in Rust and Python.
- [chart-patterns](https://github.com/focus1691/chart-patterns) | `TypeScript` | - Technical analysis library for chart patterns, price action, and volume-based pattern detection.
- [ChartScout](https://chartscout.io) - Real-time crypto chart pattern detection and alerts


### Pricing

- [Quantlib](https://www.quantlib.org)
  - [PyQL](https://github.com/enthought/pyql) | `Python`, `Cython` | - Python wrapper of the famous pricing library QuantLib
  - [QuantLib.jl](https://github.com/pazzo83/QuantLib.jl) | `Julia` | - Quantlib implementation in pure Julia.
- [FinancePy](https://github.com/domokane/FinancePy) | `Python` | - A Python Finance Library that focuses on the pricing and risk-management of Financial Derivatives, including fixed-income, equity, FX and credit derivatives.
- [tf-quant-finance](https://github.com/google/tf-quant-finance) - High-performance TensorFlow library for quantitative finance from Google
- [vollib](https://github.com/vollib/vollib) | `Python` | - Fundamentally a swig/python wrapper around Peter Jaeckel's lets_be_rational. lets_be_rational focuses exclusively on Black76, while Vollib extends this to add support for Black-Scholes and Black-Scholes-Merton.

### Risk

- [pyfolio](https://github.com/quantopian/pyfolio) | `Python` | - Portfolio and risk analytics in Python

### Optimization

- [cvxportfolio](https://github.com/cvxgrp/cvxportfolio) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cvxgrp/cvxportfolio/main) ![GitHub Repo stars](https://img.shields.io/github/stars/cvxgrp/cvxportfolio?style=social) | Python | - Portfolio optimization and back-testing.
- [skfolio](https://github.com/skfolio/skfolio) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/skfolio/skfolio/main)  ![GitHub Repo stars](https://img.shields.io/github/stars/skfolio/skfolio?style=social) | Python | - Python library for portfolio optimization built on top of scikit-learn
- [Riskfolio-Lib](https://github.com/dcajasn/Riskfolio-Lib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dcajasn/Riskfolio-Lib/master) ![GitHub Repo stars](https://img.shields.io/github/stars/dcajasn/Riskfolio-Lib?style=social) | C++, Python | - Portfolio Optimization and Quantitative Strategic Asset Allocation in Python
- [Deepdow](https://github.com/jankrepl/deepdow) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/jankrepl/deepdow/master) ![GitHub Repo stars](https://img.shields.io/github/stars/jankrepl/deepdow?style=social) | Python | - Python package connecting portfolio optimization and deep learning. Its goal is to facilitate research of networks that perform weight allocation in one forward pass.
- [PyPortfolioOpt](https://github.com/robertmartin8/PyPortfolioOpt) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/robertmartin8/PyPortfolioOpt/master) ![GitHub Repo stars](https://img.shields.io/github/stars/robertmartin8/PyPortfolioOpt?style=social) | Python | - Financial portfolio optimizations in python, including classical efficient frontier, Black-Litterman, Hierarchical Risk Parity
- [empyrial](https://github.com/ssantoshp/Empyrial) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/ssantoshp/Empyrial/main) ![GitHub Repo stars](https://img.shields.io/github/stars/ssantoshp/Empyrial?style=social) | Python | - Empyrial is a Python-based open-source quantitative investment library dedicated to financial institutions and retail investors, officially released in March 2021.
- [spectre](https://github.com/Heerozh/spectre) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Heerozh/spectre/master) ![GitHub Repo stars](https://img.shields.io/github/stars/Heerozh/spectre?style=social) | Python | - spectre is a GPU-accelerated Parallel quantitative trading library, focused on performance.

### TimeSeries Analysis

- [tsfresh](https://github.com/blue-yonder/tsfresh) - Automatic extraction of relevant features from time series.
- [Facebook Prophet](https://github.com/facebook/prophet) - Tool for producing high quality forecasts for time series data that has multiple seasonality with linear or non-linear growth.
- [pmdarima](https://github.com/alkaline-ml/pmdarima) - A statistical library designed to fill the void in Python's time series analysis capabilities, including the equivalent of R's auto.arima function.

## Visualization

- [Matplotlib](https://github.com/matplotlib/matplotlib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/matplotlib/matplotlib/main) ![GitHub Repo stars](https://img.shields.io/github/stars/matplotlib/matplotlib?style=social) | Python | - matplotlib: plotting with Python
- [Seaborn](https://github.com/mwaskom/seaborn) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/mwaskom/seaborn/master) ![GitHub Repo stars](https://img.shields.io/github/stars/mwaskom/seaborn?style=social) | Python | - Statistical data visualization in Python
- [Dash](https://github.com/plotly/dash) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/plotly/dash/master) ![GitHub Repo stars](https://img.shields.io/github/stars/plotly/dash?style=social) | Python | - Data Apps & Dashboards for Python. No JavaScript Required.
- [Perspective](https://github.com/finos/perspective) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/finos/perspective/master) ![GitHub Repo stars](https://img.shields.io/github/stars/finos/perspective?style=social) | C++, Python | - A data visualization and analytics component, especially well-suited for large and/or streaming datasets.
- [Streamlit](https://github.com/streamlit/streamlit) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/streamlit/streamlit/master) ![GitHub Repo stars](https://img.shields.io/github/stars/streamlit/streamlit?style=social) | Python | - Streamlit — A faster way to build and share data apps.
- [gradio](https://github.com/gradio-app/gradio/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/gradio-app/gradio/main) ![GitHub Repo stars](https://img.shields.io/github/stars/gradio-app/gradio?style=social) | Python | - Build and share delightful machine learning apps, all in Python.
- [pylatex](https://github.com/JelteF/PyLaTeX/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/JelteF/PyLaTeX/master) ![GitHub Repo stars](https://img.shields.io/github/stars/JelteF/PyLaTeX?style=social) | Python | - A Python library for creating LaTeX files
- [D-Tale (Man Group)](https://github.com/man-group/dtale) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/man-group/dtale/master) ![GitHub Repo stars](https://img.shields.io/github/stars/man-group/dtale?style=social) | JavaScript, Python | - D-Tale is the combination of a Flask back-end and a React front-end to bring you an easy way to view & analyze Pandas data structures.
- [mplfinance](https://github.com/matplotlib/mplfinance) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/matplotlib/mplfinance/master) ![GitHub Repo stars](https://img.shields.io/github/stars/matplotlib/mplfinance?style=social) | Python | - Financial Markets Data Visualization using Matplotlib
- [btplotting](https://github.com/happydasch/btplotting) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/happydasch/btplotting/master) ![GitHub Repo stars](https://img.shields.io/github/stars/happydasch/btplotting?style=social) | Python, bokeh | - btplotting provides plotting for backtests, optimization results and live data from backtrader.

## Message Queues

- [Kafka](https://github.com/apache/kafka) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/apache/kafka/master) ![GitHub Repo stars](https://img.shields.io/github/stars/apache/kafka?style=social) | Java | - Mirror of Apache Kafka
- [RedPanda](https://github.com/redpanda-data/redpanda/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/redpanda-data/redpanda/main) ![GitHub Repo stars](https://img.shields.io/github/stars/redpanda-data/redpanda?style=social) | C++ | - Redpanda is a streaming data platform for developers. Kafka API compatible. 10x faster. No ZooKeeper. No JVM!
- [BlazingMQ](https://github.com/bloomberg/blazingmq) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/bloomberg/blazingmq/main) ![GitHub Repo stars](https://img.shields.io/github/stars/bloomberg/blazingmq?style=social) | C++ | - A modern high-performance open source message queuing system

## Databases

- [ArcticDB (Man Group)](https://github.com/man-group/ArcticDB) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/man-group/ArcticDB/master)| `C++`, `Python` | - ArcticDB is a high performance, serverless DataFrame database built for the Python Data Science ecosystem.
- [DuckDB](https://github.com/duckdb/duckdb) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/duckdb/duckdb/main) | `C++`, `Python` | - ArcticDB is a high performance, serverless DataFrame database built for the Python Data Science ecosystem.
- [pylance](https://github.com/lancedb/lance) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/lancedb/lance/main)  | `Rust` | - Modern columnar data format for ML and LLMs implemented in Rust. Convert from parquet in 2 lines of code for 100x faster random access, vector index, and data versioning. Compatible with Pandas, DuckDB, Polars, Pyarrow
- [Arctic (Man Group)](https://github.com/man-group/arctic) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/man-group/arctic/master) | `Python` | - High performance datastore for time series and tick data
- [PyStore](https://github.com/ranaroussi/pystore) | `Python` | - Fast data store for Pandas time-series data
- [Marketstore](https://github.com/alpacahq/marketstore) | `Go` | - DataFrame Server for Financial Timeseries Data
- [Tectonicdb](https://github.com/0b01/tectonicdb) | `Rust` | - Tectonicdb is a fast, highly compressed standalone database and streaming protocol for order book ticks.
- [Redis](https://github.com/redis/redis) | `C` | - Redis is an in-memory database that persists on disk.
- [kdb](https://github.com/KxSystems) | `q` | - Companion files to kdb+ and q

## Data Source

### Stocks and General

- [* OpenBB Terminal](https://github.com/OpenBB-finance/OpenBBTerminal) | `Python` | - Investment Research for Everyone, Anywhere.
- [FinanceDatabase](https://github.com/JerBouma/FinanceDatabase) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/JerBouma/FinanceDatabase/main) - This is a database of 300.000+ symbols containing Equities, ETFs, Funds, Indices, Currencies, Cryptocurrencies and Money Markets.
- [AkShare](https://github.com/akfamily/akshare) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/akfamily/akshare/main) |`Python`| - AKShare is an elegant and simple financial data interface library for Python, built for human beings! 开源财经数据接口库
- [多因子模型数据](https://github.com/hugo2046/GetAstockFactors) - 获取经典的量化多因子模型数据
- [findatapy](https://github.com/cuemacro/findatapy) |`Python`| - findatapy creates an easy to use Python API to download market data from many sources including Quandl, Bloomberg, Yahoo, Google etc. using a unified high level interface.  
- [yfinance](https://github.com/ranaroussi/yfinance) |`Python`| - yfinance offers a threaded and Pythonic way to download market data from Yahoo!Ⓡ finance.
- [pandas-datareader](https://github.com/pydata/pandas-datareader) |`Python`| - Up to date remote data access for pandas, works for multiple versions of pandas.
- [Wallstreet](https://github.com/mcdallas/wallstreet) |`Python`| - Wallstreet: Real time Stock and Option tools
- [TuShare](https://github.com/waditu/tushare) |`Python`| - TuShare is a utility for crawling historical data of China stocks
- [Investpy](https://github.com/alvarobartt/investpy) - Financial Data Extraction from Investing.com with Python
- [awesome-data](https://github.com/akfamily/awesome-data) - Awesome-data shows most interesting data-source around the financial world
- [Fundamental Analysis Data](https://github.com/JerBouma/FundamentalAnalysis) | `Python` | - Fully-fledged Fundamental Analysis package capable of collecting 20 years of Company Profiles, Financial Statements, Ratios and Stock Data of 20.000+ companies.
- [Financial Data](https://financialdata.net/) - Stock Market and Financial Data API
- [goMacro.ai](https://gomacro.ai) - AI-powered economic calendar with institutional-grade macro insights and scenario planning for NFP, CPI, PPI and other data releases.

### Alternative

- [SEC EDGAR Filing API](https://github.com/janlukasschroeder/sec-api-python)

### Crypto

- [Cryptofeed](https://github.com/bmoscon/cryptofeed) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/bmoscon/cryptofeed/master)|`Python`| - Cryptocurrency Exchange Websocket Data Feed Handler with Asyncio
- [Orderflow](https://github.com/focus1691/orderflow) | `TypeScript`, `NestJS`, `TimescaleDB` | - Builds real-time Footprint Candles from WebSocket trade data across crypto exchanges.

## Broker APIs

- [Ib_insync](https://github.com/erdewit/ib_insync) | `Python` | - Python sync/async framework for Interactive Brokers API
- [PENDAX](https://github.com/CompendiumFi/PENDAX-SDK) | `JavaScript` | - A free Javascript library allowing simplified interaction with trading and data commands on a growing list of cryptocurrency exchanges like FTX, OKX, ByBit, & more.
- [ccxt](https://github.com/ccxt/ccxt) | `Python`, `JavaScript` | - A JavaScript / Python / PHP cryptocurrency trading API with support for more than 100 bitcoin/altcoin exchanges
- [Coinnect](https://github.com/hugues31/coinnect) | `Rust` | - Coinnect is a Rust library aiming to provide a complete access to main crypto currencies exchanges via REST API.
- [async_rithmic](https://github.com/rundef/async_rithmic) | `Python` | - Python async framework for Rithmic Protocol Buffer API
- [pmxt](https://github.com/pmxt-dev/pmxt) | `Python`, `JavaScript` | - A JavaScript / Python prediction markets trading API with support for major exchanges. (The ccxt for prediction markets)
- More is coming... (PR welcome)

## Quant Shops Code and Blog

- **JaneStreet**
  - [JaneStreet Tech Blog](https://blog.janestreet.com/)
  - [JaneStreet Tech Podcast](https://signalsandthreads.com/)
  - [Tech Talks](https://www.janestreet.com/tech-talks/)
  - [Open Source](https://opensource.janestreet.com/)
    - Tech stack: Ocaml, C, F#
- **Man AHL**
  - [Man Group Tech Blog](https://www.man.com/tech-articles-all)
  - [Podcast](https://www.man.com/maninstitute/podcasts)
  - [Open Source](https://github.com/man-group)
    - Tech stack: Python, JavaScript, Java, C, Go
- **DE Show**
  - [Open Source](https://github.com/deshaw)
    - Tech stack: Python, TypeScript, JavaScript, Rust, Nix
- **Two Sigma**
  - [Two Signal Engineering](https://www.twosigma.com/topic/engineering/)
  - [Open Source](https://github.com/twosigma)
    - Tech stack: Python, Java, C, Clojure, Rust
- **Hudson River Trading - HRT**
  - [HRT Engineering](https://www.hudsonrivertrading.com/hrtbeat/category/engineering/)

## Resources

### Research

- [RevenPack - Insight](https://www.ravenpack.com/insights/research/)
- [Alexandria Technology - Insight](https://www.alexandriatechnology.com/insights)

### Books

- [Building Low Latency Applications with C++](https://www.packtpub.com/product/building-low-latency-applications-with-c/9781837639359)
  - Source Code: <https://github.com/PacktPublishing/Building-Low-Latency-Applications-with-CPP>
- [Quantitative Portfolio Management: The Art and Science of Statistical Arbitrage (2021)](https://www.amazon.co.uk/Quantitative-Portfolio-Management-Statistical-Arbitrage/dp/1119821320/ref=asc_df_1119821320/?tag=googshopuk-21&linkCode=df0&hvadid=534858257189&hvpos=&hvnetw=g&hvrand=3040398248892159445&hvpone=&hvptwo=&hvqmt=&hvdev=c&hvdvcmdl=&hvlocint=&hvlocphy=9044954&hvtargid=pla-919734400242&psc=1&th=1&psc=1)
- [Algorithmic Trading with Python (2020) by Chris Conlan](https://github.com/chrisconlan/algorithmic-trading-with-python)
- [Python for Algorithmic Trading (2020) by Dr. Yves J. Hilpisch](https://github.com/yhilpisch/py4at)
- [Systematic Trading: A unique new method for designing trading and investing systems by Robert Carver](https://github.com/robcarver17/pysystemtrade)
- [Machine Learning for Algorithmic Trading: Predictive models to extract signals from market and alternative data for systematic trading strategies with Python](https://github.com/stefan-jansen/machine-learning-for-trading)
- [Advances in Financial Machine Learning](https://github.com/BlackArbsCEO/Adv_Fin_ML_Exercises)
- [Machine Learning for Asset Managers](https://github.com/emoen/Machine-Learning-for-Asset-Managers)

### Blogs

- [QuantBox - Systematic trading toolbox](https://quant.funcoder.net/)
- [大富翁量化](https://github.com/zillionare/zillionare)
- [Proof Engineering: The Algorithmic Trading Platform](https://medium.com/prooftrading/proof-engineering-the-algorithmic-trading-platform-b9c2f195433d)

### Tutorials

- [Algorithmic Trading for Cryptocurrencies in Python](https://github.com/tudorelu/tudorials/tree/master/trading) - A simple yet practical experiment tutorial for cryto trading.

### Courses

- [Hudson and Thames Quantitative Research](https://github.com/hudson-and-thames) - Our mission is to promote the scientific method within investment management by codifying frameworks, algorithms, and best practices.
- More is coming... (PR welcome)

## Relevant Projects

- [量化交易知识集 @ 泛程序员](https://github.com/wangzhe3224/systematic-trading-knowledge-collection) - Collect knowledge around systematic trading, including software design, trading strategies, statistical skill. 量化交易/系统化交易知识集
- [Awesome Quant 中文](https://github.com/thuquant/awesome-quant) -  中国的Quant相关资源索引
- [awesome-deep-trading](https://github.com/cbailes/awesome-deep-trading) - List of awesome resources for machine learning-based algorithmic trading
- [Awesome Crypto Trading Bots](https://github.com/botcrypto-io/awesome-crypto-trading-bots)
