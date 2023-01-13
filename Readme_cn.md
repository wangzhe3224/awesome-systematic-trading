# 棒棒的系统化交易

> 量化交易 + 一点数据科学

[![Awesome](https://awesome.re/badge.svg)](https://awesome.re)

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Java](https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=java&logoColor=white)
![C++](https://img.shields.io/badge/c++-%2300599C.svg?style=for-the-badge&logo=c%2B%2B&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Jupyter Notebook](https://img.shields.io/badge/jupyter-%23FA0F00.svg?style=for-the-badge&logo=jupyter&logoColor=white)
[Start Tracker](https://seladb.github.io/StarTrack-js/#/preload?r=wangzhe3224,awesome-systematic-trading)

[English Version](./Readme.md)

> Open access: all rights granted for use and re-use of any kind, by anyone, at no cost, under your choice of either the free MIT License or Creative Commons CC-BY International Public License.
>
> © 2021 Zhe Wang ([知乎](https://www.zhihu.com/people/wangzhetju) | [wangzhetju@gmail.com](mailto:wangzhetju@gmai.com))

我们怎么选项目呢？

- 属于系统交易和量化交易范畴
- 现在还在积极维护的
- 具有优秀的编程书写风格和软件架构
- (不全是) 有覆盖到测试

 **如果你找到好的项目，或者想删除过时的项目，可以 raise PR 哦!**

可以用快捷查找方式找到相关的libraries，比如：`Ctrl+F`, `Rust`

 关于crypto相关的: [>> 点这里 Systematic Crypto](crypto_focus.md).

- [棒棒的系统化交易](#棒棒的系统化交易)
  - [回测 + 实时交易](#回测--实时交易)
    - [General purpose](#general-purpose)
    - [加密货币相关](#加密货币相关)
    - [机器学习/强化学习](#机器学习强化学习)
  - [Alpha 集合](#alpha-集合)
    - [Alpha](#alpha)
    - [套利 (Crypto)](#套利-crypto)
  - [基本日常用到的库集合](#基本日常用到的库集合)
    - [基础库](#基础库)
    - [Computation Graph计算图](#computation-graph计算图)
    - [其他可用的库 libraries](#其他可用的库-libraries)
      - [Numpy Alternatives](#numpy-alternatives)
      - [Pandas Alternatives](#pandas-alternatives)
  - [分析工具 Analytic tools](#分析工具-analytic-tools)
    - [指标计算 Metrics computation](#指标计算-metrics-computation)
    - [指标 Indicators](#指标-indicators)
    - [定价 Pricing](#定价-pricing)
    - [风险 Risk](#风险-risk)
    - [优化 Optimization](#优化-optimization)
    - [时间序列分析 TimeSeries Analysis](#时间序列分析-timeseries-analysis)
  - [可视化 Visualization](#可视化-visualization)
  - [数据库 Databases](#数据库-databases)
  - [数据源 Data Source](#数据源-data-source)
    - [股票和其他 Stocks and General](#股票和其他-stocks-and-general)
    - [Crypto](#crypto)
  - [Broker APIs](#broker-apis)
  - [资源 Resources](#资源-resources)
    - [书籍](#书籍)
    - [博客](#博客)
    - [教程 Tutorials](#教程-tutorials)
    - [专业课程 Courses](#专业课程-courses)
  - [相关项目 Relevant Projects](#相关项目-relevant-projects)
  - [Buy me a coffee?](#buy-me-a-coffee)


## 回测 + 实时交易

### General purpose

> 事件驱动框架

Note: 如果标有`Live Trading` 表示具有实时交易功能（至少一个），否则的话，只有回测功能而已。

- [aat](https://github.com/AsyncAlgoTrading/aat) | `Python`, `C++`, `Live Trading`| - 一个异步的、事件驱动的框架，主要用于编写Python算法交易策略（可以选择C++作为加速）。其具有模块化，可扩展性，支持多种工具和策略的特点，同时支持多个交易所之间进行实时交易。
- [barter-rs](https://gitlab.com/open-source-keir/financial-modelling/trading/barter-rs) | `Rust` | - 开源Rust框架，用于建立事件驱动实时交易&回测系统。Open-source Rust framework for building event-driven live-trading & backtesting systems. Algorithmic trade with the peace of mind that comes from knowing your strategies have been backtested with a near-identical trading Engine.
- [backtesting.py](https://github.com/kernc/backtesting.py) | `Python` | - Backtesting.py 是一个 Python 框架，用于根据历史（过去）数据推断交易策略的可行性。 在 Backtrader 的基础上进行了改进，且与其他的替代方案相比，Backtesting.py 是轻量级的、快速的、用户友好的、直观的、交互式的、智能的，并且有望面向未来。
- [backtrader](https://github.com/mementum/backtrader) | `Python`, `Live Trading` | - 用于交易策略的事件驱动 Python 回测库
- [FlashFunk](https://github.com/HFQR/FlashFunk) | `Rust` | -  High Performance Runtime in Rust
- [finmarketpy](https://github.com/cuemacro/finmarketpy) | `Python` | - 用于回测交易策略和分析金融市场的 Python 库(formerly pythalesians)
- [gobacktest](https://github.com/gobacktest/gobacktest) | `Go` | - 基于Go的事件驱动回测框架
- [lumibot](https://github.com/Lumiwealth/lumibot/tree/8da88cadfe9ee35399dd69c94aa5ed3cf995f417) | `Python` | - 一个非常简单但有用的回测和基于样本的实时交易框架（运行起来有点慢......）
- [nautilus_trader](https://github.com/nautechsystems/nautilus_trader) | `Python`, `Cython`, `Rust`, `Live Trading` | - 高性能算法交易平台和事件驱动回测器
- [QuantConnect](https://github.com/QuantConnect/Lean) | `C#`, `.NET`, `Live Trading` | - Lean 算法交易引擎 by QuantConnect (Python, C#)
- [QUANTAXIS](https://github.com/QUANTAXIS/QUANTAXIS) | `Python`, `Rust`, `Live Trading` | - QUANTAXIS 支持任务调度 分布式部署的 股票/期货/期权/港股/虚拟货币 数据/回测/模拟/交易/可视化/多账户 纯本地量化解决方案
- [Rqalpha](https://github.com/ricequant/rqalpha) | `Python` | - 一个可扩展、可替换的 Python 算法回测 && 交易框架，支持多种证券
- [quanttrader](https://github.com/letianzj/quanttrader) | `Python` | - 一个完全的基于python的事件驱动回测和量化交易者的实时交易库。
- [sdoosa-algo-trade-python](https://github.com/sreenivasdoosa/sdoosa-algo-trade-python) | `Python` | - 该项目主要面向有兴趣学习使用 python 解释器编写自己的交易算法的算法交易新手。
- [vnpy](https://github.com/vnpy/vnpy) | `Python`, `Stock`, `Futures`, `Crypto`, `Live Trading` | - 基于Python的开源量化交易系统开发框架，2015年1月正式发布，逐步成长为功能齐全的量化交易平台
- [WonderTrader](https://github.com/wondertrader/wondertrader) | `C++`, `Python` | - WonderTrader——量化研发交易一站式框架
- [zvt](https://github.com/zvtvz/zvt) | `Python`, `Stock`, `Backtest` | - Modular quant framework
- [zipline](https://github.com/quantopian/zipline) | `Python` | - Zipline 是一个 Pythonic 算法交易库，用于回测的事件驱动系统。

> Vector Based Frameworks

- [bt](https://github.com/pmorissette/bt) | `Python` | -  bt是一个基于Python的灵活回测框架，用于Algo和策略树中。
- [pysystemtrade](https://github.com/robcarver17/pysystemtrade) | `Python`, `Live Trading` | - <Systematic Trading> by Rob Carver这本书的系统交易代码实现
- [vectorbt](https://github.com/polakowo/vectorbt) | `Python`, `numba` | - vectorbt 采用了一种新颖的回测方法：它完全在 pandas 和 NumPy 对象上运行，并由 Numba 加速对大规模的数据进行分析，这使得于几秒钟内测试数千种策略。

### 加密货币相关

- [bTrader](https://github.com/gabriel-milan/btrader) | `Rust` | - Binance三角套利交易机器人
- [crypto-crawler-rs](https://github.com/crypto-crawler/crypto-crawler-rs) | `Rust` | - 从加密货币交易所抓取订单簿和交易消息
- [cryptotrader-core](https://github.com/monomadic/cryptotrader-core) | `Rust` | - 简单易用的Crypto Exchange REST API client(in Rust)
- [openlimits](https://github.com/nash-io/openlimits) | `Rust` | - 一个 Rust 高性能加密货币交易 API，支持多个交易所和语言包装器。
- [Freqtrade](https://github.com/freqtrade/freqtrade) | `Python` | - Freqtrade 是一个用 Python 编写的免费开源加密交易机器人。 它旨在支持所有主要交易所并通过 Telegram 进行控制。 它包含回测、绘图和资金管理工具以及通过机器学习进行的策略优化。
- [Hummingbot](https://github.com/CoinAlpha/hummingbot) | `Python`, `Cython`, `Live Trading` | - A client for crypto market making
- [Jesse](https://github.com/jesse-ai/jesse) | `Python` | - Jesse 是一个先进的加密交易框架，旨在简化交易策略的研究和定义。
- [OctoBot](https://github.com/Drakkar-Software/OctoBot) | `Python`, `Cython`, `Live Trading`| - 具有高级 Web 界面的用于 TA、套利和社交交易的加密货币交易机器人
- [Kelp](https://github.com/stellar/kelp) | `Go`, `Live Trading` | - Kelp 是一个免费的开源交易机器人，适用于 Stellar DEX 和 100 多个中心化交易所
- [exc](https://github.com/Nouzan/exc) | `Rust` | - Exc 是一个加密货币交易所API抽象层实现，旨在让使用者更关注于策略实现。

###  机器学习/强化学习

> ML, RL

- [FinRL](https://github.com/AI4Finance-Foundation/FinRL) | `Python` | - FinRL 是第一个展示在量化金融中应用深度强化学习的巨大潜力的开源框架。
- [QLib (Microsoft)](https://github.com/microsoft/qlib) | `Python`, `Cython` | - Qlib是一个面向AI的量化投资平台，旨在挖掘潜力，赋能研究，创造AI技术在量化投资中的价值。 使用 Qlib，您可以轻松尝试您的想法以创建更好的量化投资策略。 越来越多的 SOTA Quant 研究作品/论文在 Qlib 中发布。
- [TradingGym](https://github.com/Yvictor/TradingGym) | `Python`, `Live Trading` | - 用于训练强化学习代理或简单规则库算法的交易和回测环境。
- [Stock Trading Bot using Deep Q-Learning](https://github.com/pskrunner14/trading-bot) | `Python` | - 使用Deep Q Learning的股票交易机器人

## Alpha 集合

### Alpha

- [analyzingalpha](https://github.com/leosmigel/analyzingalpha)
- [ThetaGang](https://github.com/brndnmtthws/thetagang) - ThetaGang is an IBKR bot for collecting money
  - https://www.reddit.com/r/options/comments/a36k4j/the_wheel_aka_triple_income_strategy_explained/

### 套利 (Crypto) 

> 注意：这些项目很旧，没有维护。 我把它们放在这里只是为了展示一些加密套利的逻辑。

- [Blackbird](https://github.com/butor/blackbird) | `C++` | - 黑鸟比特币套利：多头/空头市场中性策略
- [bitcoin-arbitrage](https://github.com/maxme/bitcoin-arbitrage) | `Python` | - 比特币套利 - 机会监控器
- [R2 Bitcoin Arbitrager](https://github.com/bitrinjani/r2) | `TypeScript` | - R2 Bitcoin Arbitrager 是一个由 Node.js + TypeScript 提供支持的自动套利交易系统。

## 基本日常用到的库集合

### 基础库

- [Cvxpy](https://github.com/cvxpy/cvxpy) | `Python`, `C++` | - 用于凸优化问题的 Python 嵌入式建模语言.
- [Numpy](https://github.com/numpy/numpy) | `Python`, `C` | - 使用 Python 进行科学计算的基础库
- [Scipy](https://github.com/scipy/scipy) | `Python`, `C` | - 科学计算的基础算法(Python)
- [Pandas](https://github.com/pandas-dev/pandas) | `Python`, `Cython` | - 灵活而强大的 Python 数据分析/操作库，提供类似于 R 的数据结构、框架对象、统计函数等
- [Sikit-learn](https://github.com/scikit-learn/scikit-learn) | `Python`, `Cython` | - Machine learning in Python
- [Keras](https://github.com/keras-team/keras) | `Python` | - 最友好的强化学习(in Python)
- [TensorFlow](https://github.com/tensorflow/tensorflow) | `Python`, `C++` | - 更底层的深度学习框架
- [Pytorch](https://github.com/pytorch/pytorch) | `Python` | - 具有强大 GPU 加速功能的 Python 张量和动态神经网络
- [PyMC](https://github.com/pymc-devs/pymc) | `Python` | - Python 中的概率编程：使用 Aesara 进行贝叶斯建模和概率机器学习

### Computation Graph计算图

- [Dask](https://github.com/dask/dask) | `Python` | - 在 Python 中使用 Pandas 之类 API 进行任务调度的并行计算
- [Ray](https://github.com/ray-project/ray) | `Python`, `C++` | - An open source framework that provides a simple, universal API for building distributed applications.
- [Incremental (JaneStreet)](https://github.com/janestreet/incremental) | `Ocaml` | -  Incremental 是一个库，它为您提供了一种构建复杂计算的方法，该计算可以有效地更新以响应输入的变化，灵感来自 Umut Acar et. al。关于自调整计算。 incremental在许多应用程序中都很有用
- [GraphKit](https://github.com/yahoo/graphkit) | `Python` | - 用于创建和运行有序计算图的轻量级 Python 模块。
- [Man MDF](https://github.com/man-group/mdf) | `Python` | - Python数据流编程工具包
- [Tributary](https://github.com/timkpaine/tributary) | `Python` | - Python 流式处理反应式和数据流图

### 其他可用的库 libraries

#### Numpy Alternatives

- [ndarray](https://github.com/rust-ndarray/ndarray) | `Rust` | - ndarray: 具有数组视图、多维切片和高效操作的 N 维数组

#### Pandas Alternatives

- [Polars](https://github.com/pola-rs/polars) | `Rust`, `Python` | - Polars 是使用 Apache Arrow Columnar Format 作为内存模型在 Rust 中实现的速度极快的 DataFrames 库。
- [Vaex](https://github.com/vaexio/vaex) | `Python`, `C++` | - Out-of-Core hybrid Apache Arrow/NumPy DataFrame for Python, ML, visualization and exploration of big tabular data at a billion rows per second
- [Modin](https://github.com/modin-project/modin) | `Python` |  - Modin：通过更改一行代码来加快 Pandas 工作流程
- [Koalas](https://github.com/databricks/koalas) | `Python` | - Koalas: pandas API on Apache Spark

## 分析工具 Analytic tools

### 指标计算 Metrics computation

- [ffn](https://github.com/pmorissette/ffn) | `Python` | - Python的金融函数库
- [quantstats](https://github.com/ranaroussi/quantstats) | `Python` | - 用 Python 编写的量化投资组合分析

### 指标 Indicators

- [TA-Lib](https://ta-lib.org) | `C` | - 对金融市场数据进行技术分析
  - [Python Wrapper](https://github.com/mrjbq7/ta-lib) | `Python` |
  - [Go Port](https://github.com/markcheno/go-talib) | `Go` |
  - [Rust Wrapper](https://github.com/CLevasseur/ta-lib-rust) | `Rust` |
- [ta-rust](https://github.com/greyblake/ta-rs) | `Rust` | - Rust金融分析库
- [finta](https://github.com/peerchemist/finta) | `Python` | - Pandas常用金融技术指标
- [pandas-ta](https://github.com/twopirllc/pandas-ta) | `Python` | - Pandas TA is an easy to use library that leverages the Pandas package with more than 130 Indicators and Utility functions and more than 60 TA Lib Candlestick Patterns.

### 定价 Pricing

- [Quantlib](https://www.quantlib.org)
  - [PyQL](https://github.com/enthought/pyql) | `Python`, `Cython` | - Python wrapper of the famous pricing library QuantLib
  - [QuantLib.jl](https://github.com/pazzo83/QuantLib.jl) | `Julia` | - Quantlib implementation in pure Julia.
- [FinancePy](https://github.com/domokane/FinancePy) | `Python` | - Python 金融库，专注于金融衍生品的定价和风险管理，包括固定收益、股票、外汇和信用衍生品。
- [tf-quant-finance](https://github.com/google/tf-quant-finance) - 量化金融的高性能 TensorFlow 库。

### 风险 Risk

- [pyfolio](https://github.com/quantopian/pyfolio) | `Python` | - Python 投资组合和风险分析

### 优化 Optimization

- [Deepdow](https://github.com/jankrepl/deepdow) | `Python` | - Python package connecting portfolio optimization and deep learning. Its goal is to facilitate research of networks that perform weight allocation in one forward pass.
- [PyPortfolioOpt](https://github.com/robertmartin8/PyPortfolioOpt) | `Python` | - python 中的金融投资组合优化，包括经典有效前缘、Black-Litterman、分层风险平价策略
- [Riskfolio-Lib](https://github.com/dcajasn/Riskfolio-Lib) | `Python` | - Python 投资组合优化和量化战略资产配置
- [empyrial](https://github.com/ssantoshp/Empyrial) | `Python` | - Empyrial 是一个基于 Python 的开源量化投资库，致力于金融机构和散户投资者，于 2021 年 3 月正式发布。
- [spectre](https://github.com/Heerozh/spectre) | `Python` | - Spectre 是一个 GPU 加速的并行量化交易库，专注于性能。

### 时间序列分析 TimeSeries Analysis

- [statsmodels](http://statsmodels.sourceforge.net) - 允许用户探索数据、估计统计模型和执行统计测试的 Python 模块。
- [tsfresh](https://github.com/blue-yonder/tsfresh) - 从时间序列中自动提取相关特征。
- [Facebook Prophet](https://github.com/facebook/prophet) - 用于为具有线性或非线性增长的多个季节性的时间序列数据生成高质量预测的工具。
- [pmdarima](https://github.com/alkaline-ml/pmdarima) - 一个统计库，旨在填补 Python 时间序列分析功能的空白，包括与 R 的 auto.arima 函数等效的功能。

## 可视化 Visualization

- [D-Tale (Man Group)](https://github.com/man-group/dtale) | `JavaScript`, `Python` | - D-Tale 是 Flask 后端和 React 前端的组合，提供查看和分析 Pandas 数据结构功能。
- [mplfinance](https://github.com/matplotlib/mplfinance) | `Python` | - 使用 Matplotlib 进行金融市场数据可视化
- [btplotting](https://github.com/happydasch/btplotting) | `Python`, `bokeh` | - btplotting 提供回测、优化结果和来自 backtrader 的实时数据的绘图。

## 数据库 Databases

- [Arctic (Man Group)](https://github.com/man-group/arctic) | `Python` | - 时间序列和tick data的高性能数据存储
- [Marketstore](https://github.com/alpacahq/marketstore) | `Go` | - 金融时间序列数据的 DataFrame 服务器
- [Tectonicdb](https://github.com/0b01/tectonicdb) | `Rust` | - Tectonicdb 是一个快速、高度压缩的独立数据库和order book tick串流协议。

## 数据源 Data Source

### 股票和其他 Stocks and General

- [findatapy](https://github.com/cuemacro/findatapy) |`Python`| - findatapy 创建了一个易于使用的 Python API，使用统一的高级接口从 Quandl、Bloomberg、Yahoo、Google 等多个来源下载市场数据。
- [yfinance](https://github.com/ranaroussi/yfinance) |`Python`| - yfinance 提供了从 Yahoo!Ⓡ Finance 下载市场数据。
- [pandas-datareader](https://github.com/pydata/pandas-datareader) |`Python`| - 最新远程数据访问，适用于多个版本的 pandas。
- [Wallstreet](https://github.com/mcdallas/wallstreet) |`Python`| - Wallstreet: 实时股票和期权工具
- [TuShare](https://github.com/waditu/tushare) |`Python`| - TuShare是一个用于抓取中国股票历史数据的实用程序
- [Investpy](https://github.com/alvarobartt/investpy) - 使用 Python 从 Investing.com 提取财务数据
- [AkShare](https://github.com/akfamily/akshare) |`Python`| - AKShare 是一个优雅简洁的 Python 金融数据接口库，专为人类打造！ 开源财经数据接口库
- [Fundamental Analysis Data](https://github.com/JerBouma/FundamentalAnalysis) | `Python` | - 是一个成熟的基本面分析库，能够收集 20.000 多家公司的 20 年公司简介、财务报表、比率和股票数据。

### Crypto

- [Cryptofeed](https://github.com/bmoscon/cryptofeed) |`Python`| - Cryptocurrency Exchange Websocket Data Feed Handler with Asyncio

## Broker APIs

- [Ib_insync](https://github.com/erdewit/ib_insync) | `Python` | - P盈透证券API的同步/异步python框架
- [ccxt](https://github.com/ccxt/ccxt) | `Python`, `JavaScript` | - 一个 JavaScript / Python / PHP 加密货币交易 API，支持 100 多个比特币/山寨币交易所
- [Coinnect](https://github.com/hugues31/coinnect) | `Rust` | - Coinnect 是一个 Rust 库，旨在通过 REST API 提供对主要加密货币交易所的完整访问。
- More is coming... (欢迎PR)

## 资源 Resources

### 书籍

- [Quantitative Portfolio Management: The Art and Science of Statistical Arbitrage (2021)](https://www.amazon.co.uk/Quantitative-Portfolio-Management-Statistical-Arbitrage/dp/1119821320/ref=asc_df_1119821320/?tag=googshopuk-21&linkCode=df0&hvadid=534858257189&hvpos=&hvnetw=g&hvrand=3040398248892159445&hvpone=&hvptwo=&hvqmt=&hvdev=c&hvdvcmdl=&hvlocint=&hvlocphy=9044954&hvtargid=pla-919734400242&psc=1&th=1&psc=1)
- [Algorithmic Trading with Python (2020) by Chris Conlan](https://github.com/chrisconlan/algorithmic-trading-with-python)
- [Python for Algorithmic Trading (2020) by Dr. Yves J. Hilpisch](https://github.com/yhilpisch/py4at)
- [Systematic Trading: A unique new method for designing trading and investing systems by Robert Carver](https://github.com/robcarver17/pysystemtrade)
- [Machine Learning for Algorithmic Trading: Predictive models to extract signals from market and alternative data for systematic trading strategies with Python](https://github.com/stefan-jansen/machine-learning-for-trading)
- [Advances in Financial Machine Learning](https://github.com/BlackArbsCEO/Adv_Fin_ML_Exercises)
- [Machine Learning for Asset Managers](https://github.com/emoen/Machine-Learning-for-Asset-Managers)
- More is coming... (PR welcome)

### 博客

- [Proof Engineering: The Algorithmic Trading Platform](https://medium.com/prooftrading/proof-engineering-the-algorithmic-trading-platform-b9c2f195433d) 证明工程：算法交易平台

### 教程 Tutorials

- [Algorithmic Trading for Cryptocurrencies in Python](https://github.com/tudorelu/tudorials/tree/master/trading) - 一个简单而实用的加密交易教程。

### 专业课程 Courses

- [Hudson and Thames Quantitative Research](https://github.com/hudson-and-thames) - 目标是提倡使用编纂框架、算法和最佳实践等方式促进投资管理中的科学方法。
- More is coming... (PR welcome)

## 相关项目 Relevant Projects

- [量化交易知识集 @ 泛程序员](https://github.com/wangzhe3224/systematic-trading-knowledge-collection) - 收集有关系统交易的知识，包括软件设计、交易策略、统计技能. 量化交易/系统化交易知识集
- [Awesome Quant 中文](https://github.com/thuquant/awesome-quant) -  中国的Quant相关资源索引
- [awesome-deep-trading](https://github.com/cbailes/awesome-deep-trading) - 机器学习的算法交易的资源列表
- [Awesome Crypto Trading Bots](https://github.com/botcrypto-io/awesome-crypto-trading-bots)

## Buy me a coffee?

作者收集资料和维护项目等花费了不少心思和时间，如果你想支持一下的话，可以送我一杯咖啡呀！:)

- [Patreon](https://www.patreon.com/funcoder777)
- BTC: bc1qrjrffv7aaf5f4f6dydkt4yaukt4297vedd6w6p
- 支付宝

<img src="https://github.com/wangzhe3224/awesome-systematic-trading/blob/master/assets/IMG_0825.jpg" width="200" height="200" />
