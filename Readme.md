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

## Backtest + live trading

### General purpose

> Event Driven Frameworks

Note: the one marked as `Live Trading` has reasonable live trading support for at least 1 broker. Otherwise, backtest
 function only.

- [aat](https://github.com/AsyncAlgoTrading/aat) | `Python`, `C++`, `Live Trading`| - an asynchronous, event-driven framework for writing algorithmic trading strategies in python with optional acceleration in C++. It is designed to be modular and extensible, with support for a wide variety of instruments and strategies, live trading across (and between) multiple exchanges.
- [* barter-rs](https://github.com/barter-rs/barter-rs) | `Rust` | - Open-source Rust framework for building event-driven live-trading & backtesting systems. Algorithmic trade with the peace of mind that comes from knowing your strategies have been backtested with a near-identical trading Engine.
- [* bt](https://github.com/pmorissette/bt) | `Python` | -  Flexible backtesting for Python based on Algo and Strategy Tree
- [Better Quant](https://github.com/byrnexu/betterquant) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/byrnexu/betterquant/master)| `C++`, `Live Trading` | - Better quant today, best quant tomorrow. 💪
- [Botvana](https://github.com/featherenvy/botvana) | `Rust` | - high-performance and event-driven trading system built using Rust
- [backtrader](https://github.com/mementum/backtrader) | `Python`, `Live Trading` | - Event driven Python Backtesting library for trading strategies
- [backtesting.py](https://github.com/kernc/backtesting.py) | `Python` | - Backtesting.py is a Python framework for inferring viability of trading strategies on historical (past) data. Improved upon the vision of Backtrader, and by all means surpassingly comparable to other accessible alternatives, Backtesting.py is lightweight, fast, user-friendly, intuitive, interactive, intelligent and, hopefully, future-proof.
- [FlashFunk](https://github.com/HFQR/FlashFunk) | `Rust` | -  High Performance Runtime in Rust
- [QuantFabric](https://github.com/QuantFabric/QuantFabric) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/QuantFabric/QuantFabric/master) | `C++` | - QuantFabric是基于Linux/C++开发的中高频量化交易系统，支持中金所、郑商所、大商所、上期所、上海国际能源中心的期货业务品种交易，支持上交所、深交所的股票、债券品种交易。
- [gobacktest](https://github.com/gobacktest/gobacktest) | `Go` | - A Go implementation of event-driven backtesting framework
- [Hikyuu](https://github.com/fasiondog/hikyuu) | `C++`, `Python`| - Hikyuu Quant Framework 基于C++/Python的开源量化交易研究框架
- [lumibot](https://github.com/Lumiwealth/lumibot/tree/8da88cadfe9ee35399dd69c94aa5ed3cf995f417) | `Python` | - A very simple yet useful backtesting and sample based live trading framework (a bit slow to run...)
- [* nautilus_trader](https://github.com/nautechsystems/nautilus_trader) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nautechsystems/nautilus_trader/master)| `Python`, `Cython`, `Rust`, `Live Trading` | - A high-performance algorithmic trading platform and event-driven backtester
- [PyBroker](https://github.com/edtechre/pybroker) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/edtechre/pybroker/master) | `Python` | - Algorithmic Trading in Python with Machine Learning
- [QuantConnect](https://github.com/QuantConnect/Lean) | `C#`, `.NET`, `Live Trading` | - Lean Algorithmic Trading Engine by QuantConnect (Python, C#)
- [QUANTAXIS](https://github.com/QUANTAXIS/QUANTAXIS) | `Python`, `Rust`, `Live Trading` | - QUANTAXIS 支持任务调度 分布式部署的 股票/期货/期权/港股/虚拟货币 数据/回测/模拟/交易/可视化/多账户 纯本地量化解决方案
- [Rqalpha](https://github.com/ricequant/rqalpha) | `Python` | - A extendable, replaceable Python algorithmic backtest && trading framework supporting multiple securities
- [quanttrader](https://github.com/letianzj/quanttrader) | `Python` | - Backtest and live trading in Python. Event based. Similar to backtesting.py.
- [qf-lib](https://github.com/quarkfin/qf-lib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/quarkfin/qf-lib/master) | `Python` | - Modular Python library that provides an advanced event driven backtester and a set of high quality tools for quantitative finance. Integrated with various data vendors and brokers, supports Crypto, Stocks and Futures.
- [sdoosa-algo-trade-python](https://github.com/sreenivasdoosa/sdoosa-algo-trade-python) | `Python` | - This project is mainly for newbies into algo trading who are interested in learning to code their own trading algo using python interpreter.
- [* vnpy](https://github.com/vnpy/vnpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/vnpy/vnpy/master) | `Python`, `Stock`, `Futures`, `Crypto`, `Live Trading` | - Python-based open source quantitative trading system development framework, officially released in January 2015, has grown step by step into a full-featured quantitative trading platform
- [WonderTrader](https://github.com/wondertrader/wondertrader) | `C++`, `Python` | - WonderTrader——量化研发交易一站式框架
- [zvt](https://github.com/zvtvz/zvt) | `Python`, `Stock`, `Backtest` | - Modular quant framework
- [zipline](https://github.com/quantopian/zipline) | `Python` | - Zipline is a Pythonic algorithmic trading library. It is an event-driven system for backtesting.
- [PandoraTrader](https://github.com/pegasusTrader/PandoraTrader) | `C++` | - CTP 高频量化交易平台 C++ Trade Platform for quant developer
- [hftbacktest](https://github.com/nkaz001/hftbacktest) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nkaz001/hftbacktest/master) | `Python`, `numba` | - A high-frequency trading and market-making backtesting tool accounts for limit orders, queue positions, and latencies, utilizing full tick data for trades and order books.
- [Cipher](https://github.com/nanvel/cipher-bt) | `Python` | - Backtesting library with focus on position adjustment that allows testing complicated setups. Pythonic, extensible, well-structured, documented.

> Vector Based Frameworks

- [pysystemtrade](https://github.com/robcarver17/pysystemtrade) | `Python`, `Live Trading` | - Systematic Trading in python from book <Systematic Trading> by Rob Carver
- [finmarketpy](https://github.com/cuemacro/finmarketpy) | `Python` | - Python library for backtesting trading strategies & analyzing financial markets (formerly pythalesians)
- [vectorbt](https://github.com/polakowo/vectorbt) | `Python`, `numba` | - vectorbt takes a novel approach to backtesting: it operates entirely on pandas and NumPy objects, and is accelerated by Numba to analyze any data at speed and scale. This allows for testing of many thousands of strategies in seconds.
- [fund-strategy](https://github.com/SunshowerC/fund-strategy) | `TypeScript` | - 一个简单实用的基金投资策略分析，基金回测工具
- [fastquant](https://github.com/enzoampil/fastquant) | `Python` | - Backtest and optimize your ML trading strategies with only 3 lines of code

### Crypto currency focus

- [c-binance-future-quant](https://github.com/Melelery/c-binance-future-quant/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Melelery/c-binance-future-quant/main) | `Python` | - 低成本，高效率，简单实现的币安合约量化系统架构
- [triangular-arbitrage2](https://github.com/zlq4863947/triangular-arbitrage2) | `TypeScript` | - a server side application for perform triangular arbitrage.
- [bTrader](https://github.com/gabriel-milan/btrader) | `Rust` | - Triangle arbitrage trading bot for Binance
- [crypto-crawler-rs](https://github.com/crypto-crawler/crypto-crawler-rs) | `Rust` | - Crawl orderbook and trade messages from crypto exchanges
- [cryptotrader-core](https://github.com/monomadic/cryptotrader-core) | `Rust` | - Simple to use Crypto Exchange REST API client in rust.
- [openlimits](https://github.com/nash-io/openlimits) | `Rust` | - A Rust high performance cryptocurrency trading API with support for multiple exchanges and language wrappers.
- [Freqtrade](https://github.com/freqtrade/freqtrade) | `Python` | - Freqtrade is a free and open source crypto trading bot written in Python. It is designed to support all major exchanges and be controlled via Telegram. It contains backtesting, plotting and money management tools as well as strategy optimization by machine learning.
- [* Hummingbot](https://github.com/CoinAlpha/hummingbot) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/CoinAlpha/hummingbot/development) | `Python`, `Cython`, `Live Trading` | - A client for crypto market making
- [Jesse](https://github.com/jesse-ai/jesse) | `Python` | - Jesse is an advanced crypto trading framework which aims to simplify researching and defining trading strategies.
- [* OctoBot](https://github.com/Drakkar-Software/OctoBot) | `Python`, `Cython`, `Live Trading`| - Cryptocurrency trading bot for TA, arbitrage and social trading with an advanced web interface
- [Kelp](https://github.com/stellar/kelp) | `Go`, `Live Trading` | - Kelp is a free and open-source trading bot for the Stellar DEX and 100+ centralized exchanges
- [exc](https://github.com/Nouzan/exc) | `Rust` | - The abstraction layer of exchanges.

### Machine Learning / Reinforcement Learning Focused

> ML, RL

- [FinRL](https://github.com/AI4Finance-Foundation/FinRL) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/AI4Finance-Foundation/FinRL/master) | `Python` | - FinRL is the first open-source framework to demonstrate the great potential of applying deep reinforcement learning in quantitative finance.
- [QLib (Microsoft)](https://github.com/microsoft/qlib) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/microsoft/qlib/main) | `Python`, `Cython` | - Qlib is an AI-oriented quantitative investment platform, which aims to realize the potential, empower the research, and create the value of AI technologies in quantitative investment. With Qlib, you can easily try your ideas to create better Quant investment strategies. An increasing number of SOTA Quant research works/papers are released in Qlib.
- [TradingGym](https://github.com/Yvictor/TradingGym) | `Python`, `Live Trading` | - Trading and Backtesting environment for training reinforcement learning agent or simple rule base algo.
- [Stock Trading Bot using Deep Q-Learning](https://github.com/pskrunner14/trading-bot) | `Python` | - Stock Trading Bot using Deep Q-Learning

## Alpha Collections

### General Alpha

- [Python quantitative trading strategies including VIX Calculator, Pattern Recognition, Commodity Trading Advisor, Monte Carlo, Options Straddle, Shooting Star, London Breakout, Heikin-Ashi, Pair Trading, RSI, Bollinger Bands, Parabolic SAR, Dual Thrust, Awesome, MACD](https://github.com/je-suis-tm/quant-trading#15-vix-calculator)
- [analyzingalpha](https://github.com/leosmigel/analyzingalpha)
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

- [Cvxpy](https://github.com/cvxpy/cvxpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cvxpy/cvxpy/master) | `Python`, `C++` | - A Python-embedded modeling language for convex optimization problems.
- [Numpy](https://github.com/numpy/numpy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numpy/numpy/main)| `Python`, `C` | - The fundamental package for scientific computing with Python
- Modelling
  - [Scipy](https://github.com/scipy/scipy) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/scipy/scipy/main)| `Python`, `C` | - Fundamental algorithms for scientific computing in Python
  - [statsmodels](https://github.com/statsmodels/statsmodels/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/statsmodels/statsmodels/main) - Python module that allows users to explore data, estimate statistical models, and perform statistical tests.
  - [PyMC](https://github.com/pymc-devs/pymc) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/statsmodels/statsmodels/main)| `Python` | - Probabilistic Programming in Python: Bayesian Modeling and Probabilistic Machine Learning with Aesara
- DataFrame
  - [Pandas](https://github.com/pandas-dev/pandas) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pandas-dev/pandas/main) | `Python`, `Cython` | - Flexible and powerful data analysis / manipulation library for Python, providing labeled data structures similar to R data.frame objects, statistical functions, and much more
  - [Polars](https://github.com/pola-rs/polars) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pola-rs/polars/main)| `Rust`, `Python` | - Polars is a blazingly fast DataFrames library implemented in Rust using Apache Arrow Columnar Format as memory model.
- Machine Learning
  - [Sikit-learn](https://github.com/scikit-learn/scikit-learn) | `Python`, `Cython` | - Machine learning in Python
  - [Keras](https://github.com/keras-team/keras) | `Python` | - The most user friendly Deep Learning for humans in Python
  - [TensorFlow](https://github.com/tensorflow/tensorflow) | `Python`, `C++` | - More low level Deep Learning framework
  - [Pytorch](https://github.com/pytorch/pytorch) | `Python` | - Tensors and Dynamic neural networks in Python with strong GPU acceleration
  - [Hugging Face](https://github.com/huggingface/) - The AI community building the future.
  - [LangChain](https://github.com/langchain-ai/langchain) - Building applications with LLMs through composability
- DAG
  - [Rustworkx](https://github.com/Qiskit/rustworkx/tree/main) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Qiskit/rustworkx/main) | `Rust`, `Python` | - A high performance Python graph library implemented in Rust.
  - [Networkx](https://github.com/networkx/networkx) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/networkx/networkx/main) | `Python` | - Network Analysis in Python

### Computation

- [Ray](https://github.com/ray-project/ray) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/ray-project/ray/master) | `Python`, `C++` | - An open source framework that provides a simple, universal API for building distributed applications.
- [Dask](https://github.com/dask/dask) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dask/dask/main) | `Python` | - Parallel computing with task scheduling in Python with a Pandas like API
- [Spark](https://github.com/apache/spark) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/apache/spark/master) | `Scala` | - Apache Spark - A unified analytics engine for large-scale data processing
- [Hamilton](https://github.com/dagworks-inc/hamilton) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/dagworks-inc/hamilton/main)| `Python` | - A scalable general purpose micro-framework for defining dataflows. You can use it to build dataframes, numpy matrices, python objects, ML models, etc. Embed Hamilton anywhere python runs, e.g. spark, airflow, jupyter, fastapi, python scripts, etc.
- [Incremental (JaneStreet)](https://github.com/janestreet/incremental) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/janestreet/incremental/master) | `Ocaml` | - Incremental is a library that gives you a way of building complex computations that can update efficiently in response to their inputs changing, inspired by the work of Umut Acar et. al. on self-adjusting computations. Incremental can be useful in a number of applications
- [Joblib](https://github.com/joblib/joblib)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/joblib/joblib/master) | `Python` | - running Python functions as pipeline jobs
- [Tributary](https://github.com/timkpaine/tributary) | `Python` | - Streaming reactive and dataflow graphs in Python
- [GraphKit(No activity)](https://github.com/yahoo/graphkit) | `Python` | - A lightweight Python module for creating and running ordered graphs of computations.
- [Man MDF (No activity)](https://github.com/man-group/mdf) | `Python` | - Data-flow programming toolkit for Python
- [Anchors - C++(No activity)](https://github.com/oluwatimilehin/anchors) | `C++` | - C++ library for incremental computing
- [Anchors - Rust(No activity)](https://github.com/lord/anchors) | `Rust` | - self adjusting computations in rust
- [Loman (No activity)](https://github.com/janushendersonassetallocation/loman) | `Python` | - Loman is a Python library designed to allow quantitative researchers to control complex live updating calculation processes

### Python Performance Booster

- [cython](https://github.com/cython/cython) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cython/cython/master) - Cython is a Python compiler that makes writing C extensions for Python as easy as Python itself. Cython is based on Pyrex, but supports more cutting edge functionality and optimizations.
- [numba](https://github.com/numba/numba) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numba/numba/main) - NumPy aware dynamic Python compiler using LLVM
- [pybind11](https://github.com/pybind/pybind11) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pybind/pybind11/master) - Seamless operability between C++11 and Python
- [pyo3](https://github.com/PyO3/pyo3)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/numba/numba/main) - Rust bindings for the Python interpreter
- [CuPy](https://github.com/cupy/cupy/) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/cupy/cupy/main) | `Python`, `C++`, `Cython`, `Cuda` | - CuPy is an open-source array library for GPU-accelerated computing with Python. 100x Boost for some operations
- [Bottleneck](https://github.com/pydata/bottleneck) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pydata/bottleneck/master) | `Python`, `C` | - Fast NumPy array functions written in C
- [NumExpr](https://github.com/pydata/numexpr) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/pydata/numexpr/master) | `Python`, `C++` | - Fast numerical array expression evaluator for Python, NumPy, PyTables, pandas, bcolz and more
- [pandarallel](https://github.com/nalepae/pandarallel) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/nalepae/pandarallel/master)  | `Python` | - A simple and efficient tool to parallelize Pandas operations on all available CPUs

### Alternative libraries

#### Numpy Alternatives

- [ndarray](https://github.com/rust-ndarray/ndarray) | `Rust` | - ndarray: an N-dimensional array with array views, multidimensional slicing, and efficient operations
- [faer](https://github.com/sarah-ek/faer-rs)  ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/sarah-ek/faer-rs/main) | `Rust` | - Linear algebra foundation for the Rust programming language

#### Pandas Alternatives

- [Vaex](https://github.com/vaexio/vaex) | `Python`, `C++` | - Out-of-Core hybrid Apache Arrow/NumPy DataFrame for Python, ML, visualization and exploration of big tabular data at a billion rows per second
- [Modin](https://github.com/modin-project/modin) | `Python` |  - Modin: Speed up your Pandas workflows by changing a single line of code
- [Koalas](https://github.com/databricks/koalas) | `Python` | - Koalas: pandas API on Apache Spark

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

- [Deepdow](https://github.com/jankrepl/deepdow) | `Python` | - Python package connecting portfolio optimization and deep learning. Its goal is to facilitate research of networks that perform weight allocation in one forward pass.
- [PyPortfolioOpt](https://github.com/robertmartin8/PyPortfolioOpt) | `Python` | - Financial portfolio optimizations in python, including classical efficient frontier, Black-Litterman, Hierarchical Risk Parity
- [Riskfolio-Lib](https://github.com/dcajasn/Riskfolio-Lib) | `Python` | - Portfolio Optimization and Quantitative Strategic Asset Allocation in Python
- [empyrial](https://github.com/ssantoshp/Empyrial) | `Python` | - Empyrial is a Python-based open-source quantitative investment library dedicated to financial institutions and retail investors, officially released in March 2021.
- [spectre](https://github.com/Heerozh/spectre) | `Python` | - spectre is a GPU-accelerated Parallel quantitative trading library, focused on performance.

### TimeSeries Analysis

- [tsfresh](https://github.com/blue-yonder/tsfresh) - Automatic extraction of relevant features from time series.
- [Facebook Prophet](https://github.com/facebook/prophet) - Tool for producing high quality forecasts for time series data that has multiple seasonality with linear or non-linear growth.
- [pmdarima](https://github.com/alkaline-ml/pmdarima) - A statistical library designed to fill the void in Python's time series analysis capabilities, including the equivalent of R's auto.arima function.

## Visualization

- [Matplotlib](https://github.com/matplotlib/matplotlib) | `Python` | - matplotlib: plotting with Python
- [Seaborn](https://github.com/mwaskom/seaborn) | `Python` | - Statistical data visualization in Python
- [Dash](https://github.com/plotly/dash) | `Python` | - Data Apps & Dashboards for Python. No JavaScript Required.
- [Perspective](https://github.com/finos/perspective) | `C++`, `Python` | - A data visualization and analytics component, especially well-suited for large and/or streaming datasets.
- [Streamlit](https://github.com/streamlit/streamlit) | `Python` | - Streamlit — A faster way to build and share data apps.
- [pylatex](https://github.com/JelteF/PyLaTeX/) | `Python` | - A Python library for creating LaTeX files
- [D-Tale (Man Group)](ht：tps://github.com/man-group/dtale) | `JavaScript`, `Python` | - D-Tale is the combination of a Flask back-end and a React front-end to bring you an easy way to view & analyze Pandas data structures.
- [mplfinance](https://github.com/matplotlib/mplfinance) | `Python` | - Financial Markets Data Visualization using Matplotlib
- [btplotting](https://github.com/happydasch/btplotting) | `Python`, `bokeh` | - btplotting provides plotting for backtests, optimization results and live data from backtrader.

## Message Queues

- [Kafka](https://github.com/apache/kafka) | `Java` | - Mirror of Apache Kafka
- [RedPanda](https://github.com/redpanda-data/redpanda/) | `C++` | - Redpanda is a streaming data platform for developers. Kafka API compatible. 10x faster. No ZooKeeper. No JVM!
- [BlazingMQ](https://github.com/bloomberg/blazingmq) | `C++` | - A modern high-performance open source message queuing system

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

### Alternative

- [SEC EDGAR Filing API](https://github.com/janlukasschroeder/sec-api-python)

### Crypto

- [Cryptofeed](https://github.com/bmoscon/cryptofeed) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/bmoscon/cryptofeed/master)|`Python`| - Cryptocurrency Exchange Websocket Data Feed Handler with Asyncio

## Broker APIs

- [Ib_insync](https://github.com/erdewit/ib_insync) | `Python` | - Python sync/async framework for Interactive Brokers API
- [PENDAX](https://github.com/CompendiumFi/PENDAX-SDK) | `JavaScript` | - A free Javascript library allowing simplified interaction with trading and data commands on a growing list of cryptocurrency exchanges like FTX, OKX, ByBit, & more.
- [ccxt](https://github.com/ccxt/ccxt) | `Python`, `JavaScript` | - A JavaScript / Python / PHP cryptocurrency trading API with support for more than 100 bitcoin/altcoin exchanges
- [Coinnect](https://github.com/hugues31/coinnect) | `Rust` | - Coinnect is a Rust library aiming to provide a complete access to main crypto currencies exchanges via REST API.
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
