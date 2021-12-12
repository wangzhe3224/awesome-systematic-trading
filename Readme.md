# Awesome Systematic Trading

> or Quantitative Trading

[![Awesome](https://awesome.re/badge.svg)](https://awesome.re)

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Java](https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=java&logoColor=white)
![C++](https://img.shields.io/badge/c++-%2300599C.svg?style=for-the-badge&logo=c%2B%2B&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Jupyter Notebook](https://img.shields.io/badge/jupyter-%23FA0F00.svg?style=for-the-badge&logo=jupyter&logoColor=white)

A curated list of awesome libraries, packages and resources for Systematic Trading (Quantitative Trading)

How do we pick the projects?

- Fit in Systematic Trading / Quantitative Trading domain
- Under active development
- Good coding style and software architecture
- (Optional) Reasonable test coverage

Overall, I tend to pick decent or promising libraries that closely related to systematic trading instead of including as many libraries as possible.

 **Please raise a PR if you found some good fit projects for this repo or remove some outdated projects. Thanks!**

- [Awesome Systematic Trading](#awesome-systematic-trading)
  - [Backtest + live trading](#backtest--live-trading)
    - [General purpose](#general-purpose)
    - [Crypto currency focus](#crypto-currency-focus)
  - [Basic libraries](#basic-libraries)
    - [Fundamental libraries](#fundamental-libraries)
    - [Alternative libraries](#alternative-libraries)
      - [Pandas Alternatives](#pandas-alternatives)
  - [Analytic tools](#analytic-tools)
    - [Metrics compution](#metrics-compution)
    - [Indicators](#indicators)
    - [Pricing](#pricing)
    - [Risk](#risk)
    - [Optimization](#optimization)
    - [TimeSeries Analysis](#timeseries-analysis)
  - [Visualization](#visualization)
  - [Databases](#databases)
  - [Data Source](#data-source)
  - [Resources](#resources)
    - [Books](#books)
    - [Courses](#courses)

## Backtest + live trading

### General purpose

- [aat](https://github.com/AsyncAlgoTrading/aat) | `Python`, `C++` | - an asynchronous, event-driven framework for writing algorithmic trading strategies in python with optional acceleration in C++. It is designed to be modular and extensible, with support for a wide variety of instruments and strategies, live trading across (and between) multiple exchanges.
- [backtesting.py](https://github.com/kernc/backtesting.py) | `Python` | - Backtesting.py is a Python framework for inferring viability of trading strategies on historical (past) data. Improved upon the vision of Backtrader, and by all means surpassingly comparable to other accessible alternatives, Backtesting.py is lightweight, fast, user-friendly, intuitive, interactive, intelligent and, hopefully, future-proof.
- [backtrader](https://github.com/mementum/backtrader) | `Python` | - Event driven Python Backtesting library for trading strategies
- [bt](https://github.com/pmorissette/bt) | `Python` | -  Flexible backtesting for Python based on Algo and Strategy Tree
- [FinRL](https://github.com/AI4Finance-Foundation/FinRL) | `Python` | - FinRL is the first open-source framework to demonstrate the great potential of applying deep reinforcement learning in quantitative finance.
- [QLib (Microsoft)](https://github.com/microsoft/qlib) | `Python`, `Cython` | - Qlib is an AI-oriented quantitative investment platform, which aims to realize the potential, empower the research, and create the value of AI technologies in quantitative investment. With Qlib, you can easily try your ideas to create better Quant investment strategies. An increasing number of SOTA Quant research works/papers are released in Qlib.
- [QuantConnect](https://github.com/QuantConnect/Lean) | `C#`, `.NET` | - Lean Algorithmic Trading Engine by QuantConnect (Python, C#)
- [QUANTAXIS](https://github.com/QUANTAXIS/QUANTAXIS) | `Python` | - QUANTAXIS 支持任务调度 分布式部署的 股票/期货/期权/港股/虚拟货币 数据/回测/模拟/交易/可视化/多账户 纯本地量化解决方案
- [Rqalpha](https://github.com/ricequant/rqalpha) | `Python` | - A extendable, replaceable Python algorithmic backtest && trading framework supporting multiple securities
- [pysystemtrade](https://github.com/robcarver17/pysystemtrade) | `Python` | - Systematic Trading in python from book <Systematic Trading> by Rob Carver
- [vectorbt](https://github.com/polakowo/vectorbt) | `Python`, `numba` | - vectorbt takes a novel approach to backtesting: it operates entirely on pandas and NumPy objects, and is accelerated by Numba to analyze any data at speed and scale. This allows for testing of many thousands of strategies in seconds.
- [vnpy](https://github.com/vnpy/vnpy) | `Python`, `Stock`, `Futures`, `Crypto` | - Python-based open source quantitative trading system development framework, officially released in January 2015, has grown step by step into a full-featured quantitative trading platform
- [zvt](https://github.com/zvtvz/zvt) | `Python`, `Stock`, `Backtest` | - Modular quant framework
- [zipline](https://github.com/quantopian/zipline) | `Python` | - Zipline is a Pythonic algorithmic trading library. It is an event-driven system for backtesting.

### Crypto currency focus

- [Freqtrade](https://github.com/freqtrade/freqtrade) | `Python` | - Freqtrade is a free and open source crypto trading bot written in Python. It is designed to support all major exchanges and be controlled via Telegram. It contains backtesting, plotting and money management tools as well as strategy optimization by machine learning.
- [Hummingbot](https://github.com/CoinAlpha/hummingbot) | `Python`, `Cython` | - A client for crypto market making
- [Jesse](https://github.com/jesse-ai/jesse) | `Python` | - Jesse is an advanced crypto trading framework which aims to simplify researching and defining trading strategies.
- [OctoBot](https://github.com/Drakkar-Software/OctoBot) | `Python`, `Cython`| - Cryptocurrency trading bot for TA, arbitrage and social trading with an advanced web interface
- [Kelp](https://github.com/stellar/kelp) | `Go` | - Kelp is a free and open-source trading bot for the Stellar DEX and 100+ centralized exchanges

## Basic libraries

### Fundamental libraries

- [Cvxpy](https://github.com/cvxpy/cvxpy) | `Python`, `C++` | - A Python-embedded modeling language for convex optimization problems.
- [Numpy](https://github.com/numpy/numpy) | `Python`, `C` | - The fundamental package for scientific computing with Python
- [Scipy](https://github.com/scipy/scipy) | `Python`, `C` | - Fundamental algorithms for scientific computing in Python
- [Pandas](https://github.com/pandas-dev/pandas) | `Python`, `Cython` | - Flexible and powerful data analysis / manipulation library for Python, providing labeled data structures similar to R data.frame objects, statistical functions, and much more
- [Sikit-learn](https://github.com/scikit-learn/scikit-learn) | `Python`, `Cython` | - Machine learning in Python
- [Keras](https://github.com/keras-team/keras) | `Python` | - The most user friendly Deep Learning for humans in Python
- [TensorFlow](https://github.com/tensorflow/tensorflow) | `Python`, `C++` | - More low level Deep Learning framework
- [Pytorch](https://github.com/pytorch/pytorch) | `Python` | - Tensors and Dynamic neural networks in Python with strong GPU acceleration
- [PyMC](https://github.com/pymc-devs/pymc) | `Python` | - Probabilistic Programming in Python: Bayesian Modeling and Probabilistic Machine Learning with Aesara

### Alternative libraries

#### Pandas Alternatives

- [Polars](https://github.com/pola-rs/polars) | `Rust`, `Python` | - Polars is a blazingly fast DataFrames library implemented in Rust using Apache Arrow Columnar Format as memory model.
- [Dask](https://github.com/dask/dask) | `Python` | - Parallel computing with task scheduling in Python with a Pandas like API
- [Modin](https://github.com/modin-project/modin) | `Python` |  - Modin: Speed up your Pandas workflows by changing a single line of code
- [Koalas](https://github.com/databricks/koalas) | `Python` | - Koalas: pandas API on Apache Spark

## Analytic tools

### Metrics compution

- [ffn](https://github.com/pmorissette/ffn) | `Python` | - A financial function library for Python
- [quantstats](https://github.com/ranaroussi/quantstats) | `Python` | - Portfolio analytics for quants, written in Python

### Indicators

- [TA-Lib](https://ta-lib.org) | `C` | - Perform technical analysis of financial market data
  - [Python Wrapper](https://github.com/mrjbq7/ta-lib) | `Python` |
  - [Go Port](https://github.com/markcheno/go-talib) | `Go` |
  - [Rust Wrapper](https://github.com/CLevasseur/ta-lib-rust) | `Rust` |
- [finta](https://github.com/peerchemist/finta) | `Python` | - Common financial technical indicators implemented in Pandas
- [pandas-ta](https://github.com/twopirllc/pandas-ta) | `Python` | - Pandas Technical Analysis (Pandas TA) is an easy to use library that leverages the Pandas package with more than 130 Indicators and Utility functions and more than 60 TA Lib Candlestick Patterns.

### Pricing

- [Quantlib](https://www.quantlib.org)
  - [PyQL](https://github.com/enthought/pyql) | `Python`, `Cython` | - Python wrapper of the famous pricing library QuantLib
  - [QuantLib.jl](https://github.com/pazzo83/QuantLib.jl) | `Julia` | - Quantlib implementation in pure Julia.
- [FinancePy](https://github.com/domokane/FinancePy) | `Python` | - A Python Finance Library that focuses on the pricing and risk-management of Financial Derivatives, including fixed-income, equity, FX and credit derivatives.
- [tf-quant-finance](https://github.com/google/tf-quant-finance) - High-performance TensorFlow library for quantitative finance from Google

### Risk

- [pyfolio](https://github.com/quantopian/pyfolio) | `Python` | - Portfolio and risk analytics in Python

### Optimization

- [Deepdow](https://github.com/jankrepl/deepdow) | `Python` | - Python package connecting portfolio optimization and deep learning. Its goal is to facilitate research of networks that perform weight allocation in one forward pass.
- [PyPortfolioOpt](https://github.com/robertmartin8/PyPortfolioOpt) | `Python` | - Financial portfolio optimisation in python, including classical efficient frontier, Black-Litterman, Hierarchical Risk Parity
- [empyrial](https://github.com/ssantoshp/Empyrial) | `Python` | - Empyrial is a Python-based open-source quantitative investment library dedicated to financial institutions and retail investors, officially released in March 2021.
- [spectre](https://github.com/Heerozh/spectre) | `Python` | - spectre is a GPU-accelerated Parallel quantitative trading library, focused on performance.

### TimeSeries Analysis

- [statsmodels](http://statsmodels.sourceforge.net) - Python module that allows users to explore data, estimate statistical models, and perform statistical tests.
- [tsfresh](https://github.com/blue-yonder/tsfresh) - Automatic extraction of relevant features from time series.
- [Facebook Prophet](https://github.com/facebook/prophet) - Tool for producing high quality forecasts for time series data that has multiple seasonality with linear or non-linear growth.
- [pmdarima](https://github.com/alkaline-ml/pmdarima) - A statistical library designed to fill the void in Python's time series analysis capabilities, including the equivalent of R's auto.arima function.

## Visualization

- [D-Tale (Man Group)](https://github.com/man-group/dtale) | `JavaScript`, `Python` | - D-Tale is the combination of a Flask back-end and a React front-end to bring you an easy way to view & analyze Pandas data structures.
- [mplfinance](https://github.com/matplotlib/mplfinance) | `Python` | - Financial Markets Data Visualization using Matplotlib

## Databases

- [Arctic (Man Group)](https://github.com/man-group/arctic) | `Python` | - High performance datastore for time series and tick data

## Data Source

- [findatapy](https://github.com/cuemacro/findatapy) |`Python`| - findatapy creates an easy to use Python API to download market data from many sources including Quandl, Bloomberg, Yahoo, Google etc. using a unified high level interface.  
- [yfinance](https://github.com/ranaroussi/yfinance) |`Python`| - yfinance offers a threaded and Pythonic way to download market data from Yahoo!Ⓡ finance.
- [pandas-datareader](https://github.com/pydata/pandas-datareader) |`Python`| - Up to date remote data access for pandas, works for multiple versions of pandas.
- [Wallstreet](https://github.com/mcdallas/wallstreet) |`Python`| - Wallstreet: Real time Stock and Option tools
- [TuShare](https://github.com/waditu/tushare) |`Python`| - TuShare is a utility for crawling historical data of China stocks
- [Investpy](https://github.com/alvarobartt/investpy) - Financial Data Extraction from Investing.com with Python
- [AkShare](https://github.com/akfamily/akshare) |`Python`| - AKShare is an elegant and simple financial data interface library for Python, built for human beings! 开源财经数据接口库
- [Cryptofeed](https://github.com/bmoscon/cryptofeed) |`Python`| - Cryptocurrency Exchange Websocket Data Feed Handler with Asyncio

## Resources

### Books

- [Algorithmic Trading with Python (2020) by Chris Conlan](https://github.com/chrisconlan/algorithmic-trading-with-python)
- More is coming... (PR welcome)

### Courses

- [Hudson and Thames Quantitative Research](https://github.com/hudson-and-thames) - Our mission is to promote the scientific method within investment management by codifying frameworks, algorithms, and best practices.
- More is coming... (PR welcome)
