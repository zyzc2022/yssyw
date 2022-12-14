# 介绍

[莫娜占卜铺](https://www.mona-uranai.com) 主要是一个游戏《原神》内的各项数值计算以及优化系统。包括圣遗物配装优化、基于搜索的圣遗物潜力期望、多人配装优化等  

## 圣遗物优化
### 为什么需要圣遗物优化？
圣遗物数量众多，即使有推荐的套装，同套装同主属性的圣遗物也有不止一个，切副词条的好坏难以肉眼区分  
而圣遗物优化可以帮助我们枚举所有圣遗物的组合，找出使得某个目标最大的圣遗物组合，这个目标可以是某个技能的伤害最大值，可以是某几个技能的组合期望伤害，可以是单个属性的最大值，甚至可以通过代码的方式自定义。


<div style="text-align: center">
<img src="./assets/optimize.png" />
</div>


### 优势
采用A*算法与WebAssembly加速计算，在圣遗物不多的情况下，可以达到秒算，在圣遗物很多的情况下，仍然可以在数秒内得到计算结果。

此外，采用MONA-DSL可以定制优化目标、


## 圣遗物潜力
很多时候需要对圣遗物背包进行清理，一般清理的依据是通过副词条。莫娜内置了一个基于词条数的评分标准

### 有何不同？
常见的评分标准为20级圣遗物的评分标准，对于0级圣遗物的潜力评分也是基于某种启发式规则，而莫娜的潜力引擎通过优化的广度优先搜索（DFS）对低级圣遗物强化满级时可能到达的所有状态进行搜索，得到一个评分期望。更符合科学。

因此，在评分中可以看到低级圣遗物和满级圣遗物都在一起，有时低级圣遗物的评分比满级圣遗物高，这说明该低级圣遗物的期望评分高于满级圣遗物。


## MONA-DSL
DSL即Domain Specific Language，为特定领域使用的语言

MONA-DSL可以帮助你灵活地定制不同的优化目标，你可以使用内置的优化目标，也可以选择使用DSL自定义优化目标，也可以使用其他人贡献的DSL优化目标

可在 [MONA-DSL Playground](https://mona-uranai.com/playground) 实验不同的代码


## 圣遗物扫描
通过深度学习驱动的字符识别程序，以及Rust的高性能，你可以方便快速地将游戏中的圣遗物数据导出，且基本不需要担心封号风险。（唯一的风险来自于模拟鼠标操作）

[Yas 下载](https://github.com/wormtql/yas/releases)