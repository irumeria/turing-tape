# 这是命令纸带
# 说明:执行前图灵机内态-执行前纸带上的数->执行后图灵机内态-执行后纸带上的数+图灵机下一步相对于纸带运动方向(只运动一格)
# L: LEFT
# R: RIGHT
# S: STOP
# begin
0-0->0-0R
0-1->1-1R
1-0->0-0R
1-1->2-1R
2-0->3-0L
2-1->2-1R
3-0->0-1S
3-1->4-0L
4-0->5-1L
4-1->4-1L
5-0->6-0R
5-1->5-0R
6-1->7-1R
7-0->3-1R
7-1->7-0R
# end