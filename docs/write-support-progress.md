#
 
W
r
i
t
e
 
s
u
p
p
o
r
t
 
p
r
o
g
r
e
s
s




T
h
i
s
 
d
o
c
u
m
e
n
t
 
i
s
 
t
h
e
 
r
u
n
n
i
n
g
 
r
e
p
o
r
t
 
f
o
r
 
t
h
e
 
e
f
f
o
r
t
 
t
o
 
e
x
t
e
n
d
 
`
x
f
u
s
e
`
 
f
r
o
m
 
a


r
e
a
d
-
o
n
l
y
 
X
F
S
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
 
i
n
t
o
 
a
 
r
e
a
d
/
w
r
i
t
e
 
o
n
e
.
 
 
I
t
 
r
e
c
o
r
d
s
 
w
h
a
t
 
i
s


*
a
c
t
u
a
l
l
y
*
 
i
m
p
l
e
m
e
n
t
e
d
 
a
n
d
 
*
a
c
t
u
a
l
l
y
*
 
t
e
s
t
e
d
 
—
 
n
e
v
e
r
 
w
h
a
t
 
i
s
 
p
l
a
n
n
e
d
,
 
a
n
d
 
n
e
v
e
r


w
h
a
t
 
h
a
s
 
m
e
r
e
l
y
 
b
e
e
n
 
r
e
a
s
o
n
e
d
 
a
b
o
u
t
.




L
i
c
e
n
s
i
n
g
 
a
n
d
 
p
r
o
v
e
n
a
n
c
e
 
f
o
r
 
t
h
e
 
w
o
r
k
 
a
r
e
 
t
r
a
c
k
e
d
 
s
e
p
a
r
a
t
e
l
y
 
i
n


[
`
l
i
c
e
n
s
i
n
g
.
m
d
`
]
(
l
i
c
e
n
s
i
n
g
.
m
d
)
.




#
#
 
H
o
w
 
t
o
 
r
e
a
d
 
t
h
i
s
 
d
o
c
u
m
e
n
t




*
 
A
 
s
e
c
t
i
o
n
 
i
s
 
e
i
t
h
e
r
 
*
*
d
o
n
e
*
*
,
 
*
*
i
n
 
p
r
o
g
r
e
s
s
*
*
,
 
*
*
b
l
o
c
k
e
d
*
*
,
 
o
r
 
*
*
n
o
t


 
 
s
t
a
r
t
e
d
*
*
.
 
 
N
o
t
h
i
n
g
 
i
s
 
d
e
s
c
r
i
b
e
d
 
a
s
 
d
o
n
e
 
o
n
 
t
h
e
 
s
t
r
e
n
g
t
h
 
o
f
 
a
 
c
o
m
m
e
n
t
.


*
 
A
n
y
t
h
i
n
g
 
m
a
r
k
e
d
 
d
o
n
e
 
h
a
s
 
a
n
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
,
 
u
n
i
t
 
t
e
s
t
s
,
 
a
n
d
 
—
 
w
h
e
r
e
 
t
h
e


 
 
q
u
e
s
t
i
o
n
 
i
s
 
a
b
o
u
t
 
a
n
 
o
n
-
d
i
s
k
 
s
t
r
u
c
t
u
r
e
 
—
 
e
i
t
h
e
r
 
a
 
t
e
s
t
 
t
h
a
t
 
r
e
a
d
s
 
a
 
r
e
a
l


 
 
i
m
a
g
e
 
o
r
 
a
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
r
u
n
 
o
v
e
r
 
t
h
e
 
r
e
s
u
l
t
.


*
 
S
e
c
t
i
o
n
 
[
M
e
a
s
u
r
e
d
 
i
n
v
a
r
i
a
n
t
s
]
(
#
m
e
a
s
u
r
e
d
-
i
n
v
a
r
i
a
n
t
s
)
 
r
e
c
o
r
d
s
 
w
h
a
t
 
h
a
s
 
b
e
e
n


 
 
e
s
t
a
b
l
i
s
h
e
d
 
b
y
 
e
x
p
e
r
i
m
e
n
t
 
a
g
a
i
n
s
t
 
n
a
t
i
v
e
 
X
F
S
.
 
 
E
v
e
r
y
t
h
i
n
g
 
e
l
s
e
 
e
i
t
h
e
r


 
 
r
e
f
e
r
e
n
c
e
s
 
i
t
 
o
r
 
i
s
 
m
a
r
k
e
d
 
a
s
 
n
o
t
 
y
e
t
 
e
s
t
a
b
l
i
s
h
e
d
.
 
 
W
h
e
r
e
 
a
 
q
u
e
s
t
i
o
n
 
i
s


 
 
s
t
i
l
l
 
o
p
e
n
 
i
t
 
i
s
 
w
r
i
t
t
e
n
 
d
o
w
n
 
a
s
 
a
 
q
u
e
s
t
i
o
n
,
 
n
o
t
 
a
n
s
w
e
r
e
d
.


*
 
"
B
a
s
e
l
i
n
e
"
 
b
e
l
o
w
 
i
s
 
t
h
e
 
s
t
a
t
e
 
o
f
 
t
h
e
 
r
e
p
o
s
i
t
o
r
y
 
b
e
f
o
r
e
 
a
n
y
 
w
r
i
t
e
 
s
u
p
p
o
r
t
 
w
a
s


 
 
a
d
d
e
d
.




-
-
-




#
#
 
B
a
s
e
l
i
n
e




R
e
c
o
r
d
e
d
 
o
n
 
t
h
e
 
d
e
v
e
l
o
p
m
e
n
t
 
m
a
c
h
i
n
e
 
d
e
s
c
r
i
b
e
d
 
i
n
 
[
E
n
v
i
r
o
n
m
e
n
t
]
(
#
e
n
v
i
r
o
n
m
e
n
t
)
.




|
 
C
h
e
c
k
 
|
 
R
e
s
u
l
t
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|


|
 
`
c
a
r
g
o
 
b
u
i
l
d
`
 
|
 
s
u
c
c
e
e
d
s
 
|


|
 
`
c
a
r
g
o
 
t
e
s
t
 
-
-
b
i
n
s
`
 
|
 
2
1
 
p
a
s
s
e
d
,
 
1
 
i
g
n
o
r
e
d
 
|


|
 
`
c
a
r
g
o
 
c
l
i
p
p
y
 
-
-
b
i
n
s
 
-
-
 
-
D
 
w
a
r
n
i
n
g
s
`
 
|
 
c
l
e
a
n
 
|


|
 
`
c
a
r
g
o
 
f
m
t
 
-
-
 
-
-
c
h
e
c
k
`
 
(
n
i
g
h
t
l
y
,
 
a
s
 
C
I
 
r
u
n
s
 
i
t
)
 
|
 
c
l
e
a
n
 
|


|
 
`
c
a
r
g
o
 
t
e
s
t
`
 
(
w
h
o
l
e
 
s
u
i
t
e
)
 
|
 
*
*
f
a
i
l
s
 
t
o
 
c
o
m
p
i
l
e
 
o
n
 
L
i
n
u
x
*
*
,
 
s
e
e
 
b
e
l
o
w
 
|


|
 
r
e
a
d
-
o
n
l
y
 
m
o
u
n
t
 
o
f
 
a
 
g
o
l
d
e
n
 
i
m
a
g
e
 
|
 
w
o
r
k
s
 
|




#
#
#
 
E
n
v
i
r
o
n
m
e
n
t




T
h
e
 
d
e
v
e
l
o
p
m
e
n
t
 
c
o
n
t
a
i
n
e
r
 
i
s
 
L
i
n
u
x
 
(
U
b
u
n
t
u
 
2
6
.
0
4
 
o
n
 
W
S
L
2
)
,
 
w
h
i
l
e
 
t
h
e
 
p
r
o
j
e
c
t
'
s


C
I
 
a
n
d
 
i
t
s
 
h
i
s
t
o
r
i
c
 
h
o
m
e
 
a
r
e
 
F
r
e
e
B
S
D
.
 
 
T
h
r
e
e
 
c
o
n
s
e
q
u
e
n
c
e
s
:




*
 
`
p
k
g
-
c
o
n
f
i
g
`
 
a
n
d
 
`
l
i
b
f
u
s
e
-
d
e
v
`
 
a
r
e
 
a
b
s
e
n
t
,
 
s
o
 
`
f
u
s
e
r
`
'
s
 
`
l
i
b
f
u
s
e
`
 
f
e
a
t
u
r
e


 
 
c
a
n
n
o
t
 
f
i
n
d
 
`
f
u
s
e
.
p
c
`
.
 
 
B
u
i
l
d
s
 
i
n
 
t
h
i
s
 
c
o
n
t
a
i
n
e
r
 
u
s
e
 
a
 
l
o
c
a
l
 
s
t
a
n
d
-
i
n
 
f
o
r


 
 
`
p
k
g
-
c
o
n
f
i
g
`
 
p
l
u
s
 
t
h
e
 
i
n
s
t
a
l
l
e
d
 
`
l
i
b
f
u
s
e
3
`
;
 
n
o
t
h
i
n
g
 
a
b
o
u
t
 
t
h
e
 
r
e
p
o
s
i
t
o
r
y


 
 
c
h
a
n
g
e
s
 
f
o
r
 
t
h
i
s
.
 
 
O
n
 
F
r
e
e
B
S
D
,
 
w
h
e
r
e
 
`
f
u
s
e
f
s
-
l
i
b
s
`
 
a
n
d
 
`
p
k
g
c
o
n
f
`
 
a
r
e


 
 
i
n
s
t
a
l
l
e
d
,
 
`
c
a
r
g
o
 
b
u
i
l
d
`
 
w
o
r
k
s
 
d
i
r
e
c
t
l
y
.


*
 
`
t
e
s
t
s
/
i
n
t
e
g
r
a
t
i
o
n
.
r
s
`
 
a
n
d
 
`
b
e
n
c
h
e
s
/
r
e
a
d
-
a
m
p
l
i
f
i
c
a
t
i
o
n
.
r
s
`
 
o
n
l
y
 
c
o
m
p
i
l
e
 
o
n


 
 
F
r
e
e
B
S
D
 
(
`
r
e
q
u
i
r
e
_
f
u
s
e
f
s
!
`
 
i
s
 
d
e
f
i
n
e
d
 
f
o
r
 
`
t
a
r
g
e
t
_
o
s
 
=
 
"
f
r
e
e
b
s
d
"
`
 
o
n
l
y
,
 
a
n
d


 
 
t
h
e
y
 
u
s
e
 
t
h
e
 
F
r
e
e
B
S
D
 
`
s
y
s
c
t
l
:
:
S
t
a
t
f
s
`
 
A
P
I
)
.
 
 
T
h
i
s
 
i
s
 
p
r
e
-
e
x
i
s
t
i
n
g
 
a
n
d
 
i
s
 
l
e
f
t


 
 
a
l
o
n
e
.
 
 
T
h
e
 
w
r
i
t
e
 
t
e
s
t
s
 
l
i
v
e
 
i
n
 
t
h
e
i
r
 
o
w
n
 
t
e
s
t
 
t
a
r
g
e
t
,
 
`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
,


 
 
w
h
i
c
h
 
i
s
 
p
o
r
t
a
b
l
e
 
a
n
d
 
w
h
i
c
h
 
d
o
e
s
 
r
u
n
 
i
n
 
t
h
i
s
 
c
o
n
t
a
i
n
e
r
.


*
 
`
m
d
c
o
n
f
i
g
(
8
)
`
 
n
e
e
d
s
 
r
o
o
t
,
 
s
o
 
l
o
o
p
 
d
e
v
i
c
e
s
 
a
r
e
 
n
o
t
 
a
v
a
i
l
a
b
l
e
.
 
 
T
h
e
 
w
r
i
t
e
 
t
e
s
t
s


 
 
w
o
r
k
 
o
n
 
p
l
a
i
n
 
i
m
a
g
e
 
*
f
i
l
e
s
*
 
i
n
s
t
e
a
d
,
 
w
h
i
c
h
 
i
s
 
e
x
a
c
t
l
y
 
h
o
w
 
t
h
e
 
g
o
l
d
e
n
 
i
m
a
g
e
s


 
 
w
o
r
k
,
 
a
n
d
 
t
h
e
y
 
u
s
e
 
t
h
e
 
s
a
m
e
 
h
e
l
p
e
r
s
 
a
s
 
t
h
e
 
e
x
i
s
t
i
n
g
 
t
e
s
t
s
.




N
a
t
i
v
e
 
X
F
S
 
t
o
o
l
i
n
g
 
6
.
1
8
 
(
`
m
k
f
s
.
x
f
s
`
,
 
`
x
f
s
_
d
b
`
,
 
`
x
f
s
_
r
e
p
a
i
r
`
,
 
`
x
f
s
_
m
e
t
a
d
u
m
p
`
,


`
x
f
s
_
m
d
r
e
s
t
o
r
e
`
,
 
`
x
f
s
_
b
m
a
p
`
,
 
`
x
f
s
_
l
o
g
p
r
i
n
t
`
,
 
`
x
f
s
_
i
n
f
o
`
)
 
i
s
 
a
v
a
i
l
a
b
l
e
 
a
n
d
 
i
s


u
s
e
d
 
a
s
 
a
 
b
l
a
c
k
 
b
o
x
 
t
o
 
g
e
n
e
r
a
t
e
 
i
m
a
g
e
s
 
a
n
d
 
t
o
 
c
h
e
c
k
 
r
e
s
u
l
t
s
.




`
x
f
s
_
r
e
p
a
i
r
`
 
i
n
 
i
t
s
 
o
r
d
i
n
a
r
y
 
(
n
o
n
-
`
-
n
`
)
 
m
o
d
e
 
r
e
b
u
i
l
d
s
 
a
 
g
r
o
u
p
 
h
e
a
d
e
r
 
a
n
d
 
i
t
s


t
r
e
e
s
.
 
 
T
h
a
t
 
m
a
k
e
s
 
i
t
 
t
h
e
 
o
n
e
 
f
r
e
e
l
y
 
a
v
a
i
l
a
b
l
e
 
a
u
t
h
o
r
i
t
y
 
o
n
 
w
h
a
t
 
a
 
*
c
o
r
r
e
c
t
*


g
r
o
u
p
 
h
e
a
d
e
r
 
l
o
o
k
s
 
l
i
k
e
,
 
a
n
d
 
i
t
 
i
s
 
u
s
e
d
 
b
e
l
o
w
 
a
s
 
o
n
e
.




#
#
#
 
C
u
r
r
e
n
t
 
s
u
p
p
o
r
t
e
d
 
X
F
S
 
f
e
a
t
u
r
e
s
 
(
r
e
a
d
 
s
i
d
e
)




|
 
A
r
e
a
 
|
 
S
u
p
p
o
r
t
e
d
 
|


|
:
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
|


|
 
F
i
l
e
s
y
s
t
e
m
 
v
e
r
s
i
o
n
 
|
 
4
 
a
n
d
 
5
 
|


|
 
B
l
o
c
k
 
s
i
z
e
 
|
 
5
1
2
 
B
 
a
n
d
 
l
a
r
g
e
r
 
|


|
 
I
n
o
d
e
 
s
i
z
e
 
|
 
2
5
6
 
B
 
(
v
1
/
v
2
 
i
n
o
d
e
s
)
 
a
n
d
 
5
1
2
 
B
 
(
v
3
 
i
n
o
d
e
s
)
 
|


|
 
I
n
o
d
e
 
f
o
r
m
a
t
s
 
|
 
`
d
e
v
`
,
 
`
l
o
c
a
l
`
,
 
`
e
x
t
e
n
t
s
`
,
 
`
b
t
r
e
e
`
 
|


|
 
D
i
r
e
c
t
o
r
y
 
f
o
r
m
a
t
s
 
|
 
s
h
o
r
t
f
o
r
m
,
 
b
l
o
c
k
,
 
l
e
a
f
,
 
n
o
d
e
,
 
b
t
r
e
e
,
 
a
n
d
 
t
h
e
 
s
i
n
g
l
e
-
b
l
o
c
k
 
"
b
t
r
e
e
 
w
i
t
h
 
o
n
e
 
l
e
a
f
"
 
c
a
s
e
 
|


|
 
E
x
t
e
n
d
e
d
 
a
t
t
r
i
b
u
t
e
s
 
|
 
s
h
o
r
t
f
o
r
m
,
 
e
x
t
e
n
t
s
,
 
l
e
a
f
,
 
n
o
d
e
,
 
b
t
r
e
e
;
 
b
o
t
h
 
v
1
 
a
n
d
 
v
2
 
(
v
5
)
 
a
t
t
r
i
b
u
t
e
 
f
o
r
m
a
t
s
 
|


|
 
F
e
a
t
u
r
e
s
 
|
 
`
f
t
y
p
e
`
,
 
`
a
t
t
r
2
`
,
 
`
c
r
c
`
,
 
`
p
r
o
j
i
d
3
2
`
,
 
`
a
l
i
g
n
`
,
 
`
s
p
a
r
s
e
 
i
n
o
d
e
s
`
,
 
`
l
a
r
g
e
 
e
x
t
e
n
t
 
c
o
u
n
t
s
`
 
(
N
R
E
X
T
6
4
)
,
 
`
p
a
r
e
n
t
 
p
o
i
n
t
e
r
`
 
(
r
e
a
d
)
 
|


|
 
R
e
j
e
c
t
e
d
 
a
t
 
m
o
u
n
t
 
|
 
`
m
e
t
a
_
u
u
i
d
`
,
 
`
n
e
e
d
s
r
e
p
a
i
r
`
,
 
`
m
e
t
a
d
i
r
`
 
(
w
i
t
h
 
a
n
 
R
T
 
d
e
v
i
c
e
)
,
 
`
z
o
n
e
d
`
 
(
w
i
t
h
 
a
n
 
R
T
 
d
e
v
i
c
e
)
,
 
u
n
k
n
o
w
n
 
`
f
e
a
t
u
r
e
s
_
i
n
c
o
m
p
a
t
`
 
b
i
t
s
 
|


|
 
R
e
a
l
-
t
i
m
e
 
d
e
v
i
c
e
s
 
|
 
r
e
a
d
 
o
n
l
y
,
 
w
i
t
h
 
a
 
s
e
p
a
r
a
t
e
 
R
T
 
d
e
v
i
c
e
 
a
r
g
u
m
e
n
t
 
|




#
#
#
 
C
u
r
r
e
n
t
 
F
U
S
E
 
o
p
e
r
a
t
i
o
n
s




I
m
p
l
e
m
e
n
t
e
d
:
 
`
i
n
i
t
`
,
 
`
l
o
o
k
u
p
`
,
 
`
f
o
r
g
e
t
`
,
 
`
g
e
t
a
t
t
r
`
,
 
`
r
e
a
d
l
i
n
k
`
,
 
`
o
p
e
n
`
,


`
r
e
a
d
`
,
 
`
l
s
e
e
k
`
,
 
`
o
p
e
n
d
i
r
`
,
 
`
r
e
a
d
d
i
r
`
,
 
`
s
t
a
t
f
s
`
,
 
`
g
e
t
x
a
t
t
r
`
,
 
`
l
i
s
t
x
a
t
t
r
`
,


`
w
r
i
t
e
`
,
 
`
f
l
u
s
h
`
,
 
`
f
s
y
n
c
`
,
 
`
r
e
l
e
a
s
e
`
.




N
o
t
 
i
m
p
l
e
m
e
n
t
e
d
:
 
`
s
e
t
a
t
t
r
`
,
 
`
c
r
e
a
t
e
`
,
 
`
m
k
d
i
r
`
,
 
`
u
n
l
i
n
k
`
,
 
`
r
m
d
i
r
`
,
 
`
r
e
n
a
m
e
`
,


`
s
e
t
x
a
t
t
r
`
,
 
`
r
e
m
o
v
e
x
a
t
t
r
`
,
 
`
f
a
l
l
o
c
a
t
e
`
,
 
`
l
i
n
k
`
,
 
`
s
y
m
l
i
n
k
`
,
 
`
m
k
n
o
d
`
,
 
`
a
c
c
e
s
s
`
,


`
c
h
o
w
n
`
-
f
a
m
i
l
y
.




`
F
U
S
E
_
N
O
_
O
P
E
N
_
S
U
P
P
O
R
T
`
 
a
n
d
 
`
F
U
S
E
_
N
O
_
O
P
E
N
D
I
R
_
S
U
P
P
O
R
T
`
 
a
r
e
 
n
e
g
o
t
i
a
t
e
d
,
 
s
o
 
t
h
e


k
e
r
n
e
l
 
u
s
u
a
l
l
y
 
d
o
e
s
 
n
o
t
 
s
e
n
d
 
`
o
p
e
n
`
/
`
o
p
e
n
d
i
r
`
 
a
t
 
a
l
l
.




#
#
#
 
L
i
m
i
t
a
t
i
o
n
s
 
r
e
l
e
v
a
n
t
 
t
o
 
w
r
i
t
e
 
s
u
p
p
o
r
t




1
.
 
A
 
r
e
a
d
-
w
r
i
t
e
 
m
o
u
n
t
 
i
s
 
e
x
p
e
r
i
m
e
n
t
a
l
 
a
n
d
 
n
o
t
 
c
r
a
s
h
 
s
a
f
e
:
 
i
t
 
i
s
 
r
e
a
c
h
e
d
 
o
n
l
y


 
 
 
b
e
h
i
n
d
 
`
-
-
e
x
p
e
r
i
m
e
n
t
a
l
-
r
w
`
 
a
n
d
 
c
o
m
m
i
t
s
 
b
l
o
c
k
s
 
s
t
r
a
i
g
h
t
 
t
o
 
t
h
e
 
i
m
a
g
e
 
w
i
t
h
 
n
o


 
 
 
j
o
u
r
n
a
l
.
 
 
T
h
e
r
e
 
i
s
 
n
o
 
r
e
c
o
v
e
r
y
 
f
r
o
m
 
a
 
t
o
r
n
 
w
r
i
t
e
.


2
.
 
A
 
f
i
l
e
'
s
 
d
a
t
a
 
f
o
r
k
 
m
u
s
t
 
b
e
 
a
 
l
i
s
t
 
o
f
 
e
x
t
e
n
t
s
 
i
n
 
t
h
e
 
i
n
o
d
e
.
 
 
A
 
f
i
l
e
 
w
h
o
s
e


 
 
 
f
o
r
k
 
i
s
 
a
 
B
+
t
r
e
e
 
i
s
 
r
e
f
u
s
e
d
 
w
i
t
h
 
a
 
m
e
s
s
a
g
e
 
t
h
a
t
 
s
a
y
s
 
s
o
.


3
.
 
O
v
e
r
w
r
i
t
i
n
g
,
 
e
x
t
e
n
d
i
n
g
 
a
n
d
 
t
r
u
n
c
a
t
i
n
g
 
a
r
e
 
i
m
p
l
e
m
e
n
t
e
d
.
 
 
T
h
e
r
e
 
i
s
 
n
o
 
`
c
r
e
a
t
e
`
,


 
 
 
s
o
 
a
 
w
r
i
t
e
 
n
e
v
e
r
 
h
a
s
 
t
o
 
m
a
k
e
 
a
 
n
e
w
 
d
i
r
e
c
t
o
r
y
 
e
n
t
r
y
,
 
a
n
d
 
n
o
 
`
u
n
l
i
n
k
`
,
 
s
o
 
a
 
w
r
i
t
e


 
 
 
n
e
v
e
r
 
h
a
s
 
t
o
 
t
a
k
e
 
a
 
w
h
o
l
e
 
f
i
l
e
'
s
 
b
l
o
c
k
s
 
a
w
a
y
 
a
t
 
o
n
c
e
.
 
 
`
s
e
t
a
t
t
r
`
 
i
s
 
h
o
n
o
u
r
e
d


 
 
 
o
n
l
y
 
f
o
r
 
a
 
f
i
l
e
'
s
 
s
i
z
e
;
 
a
n
y
t
h
i
n
g
 
e
l
s
e
 
i
t
 
i
s
 
a
s
k
e
d
 
t
o
 
c
h
a
n
g
e
 
i
s
 
r
e
f
u
s
e
d
 
w
i
t
h


 
 
 
`
E
N
O
T
S
U
P
`
 
r
a
t
h
e
r
 
t
h
a
n
 
a
c
k
n
o
w
l
e
d
g
e
d
 
a
n
d
 
i
g
n
o
r
e
d
.


4
.
 
T
h
e
 
r
e
v
e
r
s
e
 
m
a
p
p
i
n
g
 
t
r
e
e
,
 
t
h
e
 
r
e
f
e
r
e
n
c
e
 
c
o
u
n
t
 
t
r
e
e
,
 
t
h
e
 
i
n
o
d
e
 
a
l
l
o
c
a
t
i
o
n


 
 
 
g
r
o
u
p
'
s
 
n
e
w
-
c
h
u
n
k
 
c
o
u
n
t
e
r
s
,
 
a
n
d
 
t
h
e
 
l
o
g
 
a
r
e
 
n
o
t
 
m
a
i
n
t
a
i
n
e
d
.
 
 
A
 
r
e
a
d
-
w
r
i
t
e


 
 
 
m
o
u
n
t
 
r
e
f
u
s
e
s
 
i
m
a
g
e
s
 
t
h
a
t
 
h
a
v
e
 
f
e
a
t
u
r
e
s
 
i
t
 
c
a
n
n
o
t
 
k
e
e
p
 
u
p
 
t
o
 
d
a
t
e
.


5
.
 
E
v
e
r
y
t
h
i
n
g
 
a
b
o
v
e
 
`
t
r
a
n
s
a
c
t
i
o
n
.
r
s
`
 
w
r
i
t
e
s
 
t
o
 
t
h
e
 
i
m
a
g
e
 
t
h
r
o
u
g
h
 
a


 
 
 
`
T
r
a
n
s
a
c
t
i
o
n
`
;
 
n
o
t
h
i
n
g
 
w
r
i
t
e
s
 
t
o
 
t
h
e
 
d
e
v
i
c
e
 
d
i
r
e
c
t
l
y
.




-
-
-




#
#
 
S
t
a
t
u
s




#
#
#
 
C
o
m
p
l
e
t
e
d




|
 
A
r
e
a
 
|
 
S
t
a
t
u
s
 
|
 
W
h
e
r
e
 
|


|
:
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|


|
 
W
r
i
t
a
b
l
e
 
b
l
o
c
k
 
d
e
v
i
c
e
 
|
 
d
o
n
e
 
|
 
`
b
l
o
c
k
_
d
e
v
i
c
e
.
r
s
`
 
|


|
 
B
l
o
c
k
 
c
a
c
h
e
 
w
i
t
h
 
e
x
p
l
i
c
i
t
 
d
i
r
t
y
 
s
t
a
t
e
 
|
 
d
o
n
e
 
|
 
`
b
l
o
c
k
_
c
a
c
h
e
.
r
s
`
 
|


|
 
T
r
a
n
s
a
c
t
i
o
n
 
l
a
y
e
r
 
(
b
e
g
i
n
 
/
 
c
o
m
m
i
t
 
/
 
a
b
o
r
t
)
 
|
 
d
o
n
e
 
|
 
`
t
r
a
n
s
a
c
t
i
o
n
.
r
s
`
 
|


|
 
M
u
t
a
b
l
e
 
i
n
o
d
e
 
i
m
a
g
e
 
w
i
t
h
 
t
y
p
e
d
 
s
e
t
t
e
r
s
 
|
 
d
o
n
e
 
|
 
`
i
n
o
d
e
.
r
s
`
 
(
`
R
a
w
D
i
n
o
d
e
`
)
 
|


|
 
I
n
o
d
e
 
c
h
e
c
k
s
u
m
 
h
a
n
d
l
i
n
g
 
(
v
3
 
C
R
C
-
3
2
C
)
 
|
 
d
o
n
e
 
|
 
`
i
n
o
d
e
.
r
s
`
 
|


|
 
E
x
t
e
n
t
 
m
a
p
 
s
h
a
r
e
d
 
b
y
 
r
e
a
d
 
a
n
d
 
w
r
i
t
e
 
p
a
t
h
s
 
|
 
d
o
n
e
 
|
 
`
e
x
t
e
n
t
.
r
s
`
 
|


|
 
O
v
e
r
w
r
i
t
e
 
o
f
 
a
l
r
e
a
d
y
 
a
l
l
o
c
a
t
e
d
 
f
i
l
e
 
d
a
t
a
 
|
 
d
o
n
e
 
|
 
`
v
o
l
u
m
e
.
r
s
`
,
 
`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
 
|


|
 
W
r
i
t
e
 
c
a
p
a
b
i
l
i
t
y
 
g
a
t
e
 
|
 
d
o
n
e
 
|
 
`
c
a
p
a
b
i
l
i
t
i
e
s
.
r
s
`
 
|


|
 
A
G
F
 
d
e
c
o
d
i
n
g
 
a
n
d
 
c
h
a
n
g
e
 
i
n
 
p
l
a
c
e
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
g
f
.
r
s
`
 
|


|
 
A
G
F
L
 
d
e
c
o
d
i
n
g
 
a
n
d
 
c
h
a
n
g
e
 
i
n
 
p
l
a
c
e
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
g
f
l
.
r
s
`
 
|


|
 
F
r
e
e
-
s
p
a
c
e
 
t
r
e
e
 
w
a
l
k
i
n
g
 
(
b
o
t
h
 
t
r
e
e
s
,
 
a
l
l
 
l
e
v
e
l
s
)
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
f
r
e
e
_
s
p
a
c
e
.
r
s
`
 
|


|
 
B
N
O
 
/
 
C
N
T
 
s
e
a
r
c
h
i
n
g
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
f
r
e
e
_
s
p
a
c
e
.
r
s
`
 
|


|
 
F
r
e
e
-
s
p
a
c
e
 
l
e
a
f
 
m
u
t
a
t
i
o
n
 
(
a
d
d
 
/
 
r
e
m
o
v
e
 
/
 
r
e
-
o
r
d
e
r
)
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
f
r
e
e
_
s
p
a
c
e
.
r
s
`
 
|


|
 
A
l
l
o
c
a
t
i
o
n
 
f
r
o
m
 
e
x
i
s
t
i
n
g
 
f
r
e
e
-
s
p
a
c
e
 
r
u
n
s
,
 
b
o
t
h
 
t
r
e
e
s
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
f
r
e
e
_
s
p
a
c
e
.
r
s
`
 
|


|
 
T
r
a
n
s
a
c
t
i
o
n
a
l
 
b
l
o
c
k
 
a
l
l
o
c
a
t
i
o
n
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
A
G
F
 
f
r
e
e
-
b
l
o
c
k
 
a
n
d
 
l
o
n
g
e
s
t
-
r
u
n
 
u
p
d
a
t
e
s
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
P
u
t
t
i
n
g
 
b
l
o
c
k
s
 
b
a
c
k
 
i
n
t
o
 
b
o
t
h
 
t
r
e
e
s
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
S
i
b
l
i
n
g
 
c
h
a
i
n
s
 
k
e
p
t
 
w
e
l
l
 
f
o
r
m
e
d
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
f
r
e
e
_
s
p
a
c
e
.
r
s
`
 
|


|
 
F
r
e
e
 
l
i
s
t
:
 
a
n
 
e
m
p
t
y
 
w
i
n
d
o
w
 
h
a
n
d
s
 
o
u
t
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
F
r
e
e
 
l
i
s
t
:
 
a
 
w
i
n
d
o
w
 
t
h
a
t
 
l
i
e
s
 
i
s
 
r
e
f
u
s
e
d
,
 
n
o
t
 
a
n
s
w
e
r
e
d
 
e
l
s
e
w
h
e
r
e
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
g
f
l
.
r
s
`
 
|


|
 
F
r
e
e
 
l
i
s
t
:
 
t
h
e
 
w
i
n
d
o
w
 
i
s
 
a
 
s
l
i
c
e
,
 
a
n
d
 
i
t
s
 
f
r
o
n
t
 
i
s
 
t
h
e
 
n
e
x
t
 
e
n
t
r
y
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
g
f
l
.
r
s
`
 
|


|
 
A
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
'
s
 
o
w
n
e
r
s
h
i
p
 
a
n
d
 
t
h
e
 
c
o
u
n
t
e
r
s
 
t
h
a
t
 
r
e
c
o
r
d
 
i
t
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
I
n
o
d
e
 
e
x
t
e
n
t
 
i
n
s
e
r
t
i
o
n
 
w
i
t
h
 
j
o
i
n
i
n
g
 
|
 
d
o
n
e
 
|
 
`
i
n
o
d
e
.
r
s
`
 
|


|
 
F
i
l
e
 
e
x
t
e
n
s
i
o
n
 
b
y
 
a
 
c
o
n
t
i
g
u
o
u
s
 
r
u
n
 
|
 
d
o
n
e
 
|
 
`
v
o
l
u
m
e
.
r
s
`
,
 
`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
 
|


|
 
H
o
l
e
s
:
 
a
 
g
a
p
 
i
s
 
l
e
f
t
 
u
n
a
l
l
o
c
a
t
e
d
 
a
n
d
 
r
e
a
d
s
 
a
s
 
z
e
r
o
e
s
 
|
 
d
o
n
e
 
|
 
`
v
o
l
u
m
e
.
r
s
`
,
 
`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
 
|


|
 
S
u
p
e
r
b
l
o
c
k
 
f
r
e
e
-
b
l
o
c
k
 
t
o
t
a
l
 
f
o
l
l
o
w
s
 
t
h
e
 
g
r
o
u
p
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
E
x
i
s
t
i
n
g
-
c
h
u
n
k
 
i
n
o
d
e
 
a
l
l
o
c
a
t
i
o
n
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
I
N
O
B
T
 
c
o
n
s
i
s
t
e
n
c
y
 
v
a
l
i
d
a
t
i
o
n
 
(
`
f
r
e
e
c
o
u
n
t
 
=
=
 
p
o
p
c
o
u
n
t
(
f
r
e
e
_
m
a
s
k
)
`
)
 
|
 
d
o
n
e
 
|
 
`
a
l
l
o
c
/
i
n
o
b
t
.
r
s
`
,
 
`
a
l
l
o
c
/
a
l
l
o
c
a
t
o
r
.
r
s
`
 
|


|
 
T
h
e
 
m
e
a
s
u
r
e
d
 
i
n
v
a
r
i
a
n
t
s
 
b
e
l
o
w
 
|
 
d
o
n
e
 
|
 
t
h
i
s
 
d
o
c
u
m
e
n
t
,
 
`
a
l
l
o
c
/
`
 
t
e
s
t
s
 
|




#
#
#
 
I
n
 
p
r
o
g
r
e
s
s




|
 
A
r
e
a
 
|
 
S
t
a
t
u
s
 
|
 
W
h
a
t
 
i
s
 
m
i
s
s
i
n
g
 
|


|
:
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
F
r
e
e
-
s
p
a
c
e
 
t
r
e
e
 
s
t
r
u
c
t
u
r
a
l
 
g
r
o
w
t
h
 
|
 
i
n
 
p
r
o
g
r
e
s
s
 
|
 
L
e
a
f
 
s
p
l
i
t
 
a
n
d
 
r
o
o
t
 
s
p
l
i
t
 
e
x
i
s
t
 
a
n
d
 
a
r
e
 
t
e
s
t
e
d
 
i
n
 
m
e
m
o
r
y
.
 
 
T
h
e
y
 
a
r
e
 
n
o
t
 
r
e
a
c
h
a
b
l
e
 
f
r
o
m
 
t
h
e
 
i
m
a
g
e
,
 
b
e
c
a
u
s
e
 
r
e
a
c
h
i
n
g
 
t
h
e
m
 
n
e
e
d
s
 
a
 
l
e
a
f
 
t
o
 
o
v
e
r
f
l
o
w
,
 
a
n
d
 
n
o
 
t
e
s
t
 
c
a
n
 
o
v
e
r
f
l
o
w
 
a
 
l
e
a
f
 
h
o
n
e
s
t
l
y
 
w
i
t
h
o
u
t
 
a
 
f
i
l
e
 
g
i
v
i
n
g
 
u
p
 
i
t
s
 
b
l
o
c
k
s
 
—
 
w
h
i
c
h
 
i
s
 
t
h
e
 
t
r
u
n
c
a
t
e
 
t
h
a
t
 
i
s
 
n
o
t
 
b
u
i
l
t
.
 
|


|
 
F
r
e
e
-
s
p
a
c
e
 
t
r
e
e
 
s
h
r
i
n
k
a
g
e
 
|
 
i
n
 
p
r
o
g
r
e
s
s
 
|
 
L
e
a
f
 
m
e
r
g
i
n
g
 
w
o
r
k
s
 
a
n
d
 
i
s
 
c
h
e
c
k
e
d
 
o
n
 
a
 
r
e
a
l
 
i
m
a
g
e
 
w
i
t
h
 
`
x
f
s
_
r
e
p
a
i
r
`
.
 
 
R
o
o
t
 
c
o
l
l
a
p
s
e
 
d
o
e
s
 
n
o
t
 
e
x
i
s
t
,
 
a
n
d
 
i
s
 
n
o
t
 
r
e
a
c
h
a
b
l
e
:
 
n
o
t
h
i
n
g
 
e
m
p
t
i
e
s
 
a
n
 
i
n
t
e
r
i
o
r
 
n
o
d
e
,
 
b
e
c
a
u
s
e
 
a
 
p
a
r
e
n
t
 
a
l
w
a
y
s
 
h
a
s
 
a
t
 
l
e
a
s
t
 
t
w
o
 
c
h
i
l
d
r
e
n
 
a
n
d
 
t
h
e
 
m
e
r
g
e
 
b
r
a
n
c
h
 
r
e
f
u
s
e
s
 
t
o
 
t
a
k
e
 
t
h
e
 
l
a
s
t
 
o
n
e
.
 
|




#
#
#
 
B
l
o
c
k
e
d




*
*
F
r
e
e
 
s
p
a
c
e
 
t
r
e
e
 
g
r
o
w
t
h
,
 
b
e
c
a
u
s
e
 
n
o
t
h
i
n
g
 
c
a
n
 
r
e
a
c
h
 
i
t
 
y
e
t
.
*
*
 
 
A
 
s
p
l
i
t
 
n
e
e
d
s
 
a


m
e
t
a
d
a
t
a
 
b
l
o
c
k
;
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
s
 
n
o
w
 
c
o
m
e
 
a
n
d
 
g
o
 
w
i
t
h
 
t
h
e
 
a
c
c
o
u
n
t
i
n
g
 
X
F
S
 
e
x
p
e
c
t
s
,


o
n
 
b
o
t
h
 
p
a
t
h
s
 
a
n
d
 
i
n
 
b
o
t
h
 
d
i
r
e
c
t
i
o
n
s
.
 
 
W
h
a
t
 
i
s
 
l
e
f
t
 
i
s
 
a
 
w
a
y
 
t
o
 
m
a
k
e
 
a
 
l
e
a
f


o
v
e
r
f
l
o
w
 
o
n
 
a
 
r
e
a
l
 
i
m
a
g
e
,
 
w
h
i
c
h
 
n
e
e
d
s
 
a
 
f
i
l
e
 
t
o
 
g
i
v
e
 
u
p
 
i
t
s
 
b
l
o
c
k
s
 
—
 
t
h
e


t
r
u
n
c
a
t
e
 
t
h
a
t
 
i
s
 
n
o
t
 
b
u
i
l
t
.




W
h
a
t
 
i
s
 
l
e
f
t
 
i
n
 
o
r
d
e
r
:




0
.
 
*
*
A
n
 
i
m
a
g
e
 
`
m
k
f
s
.
x
f
s
`
 
b
u
i
l
t
 
t
h
a
t
 
h
a
s
 
c
o
n
s
u
m
e
d
 
a
 
l
i
s
t
 
e
n
t
r
y
.
*
*
 
 
N
o
t
 
c
o
d
e
:
 
n
o


 
 
 
t
e
s
t
 
c
a
n
 
b
u
i
l
d
 
o
n
e
,
 
t
h
e
 
e
n
v
i
r
o
n
m
e
n
t
 
c
a
n
n
o
t
 
m
o
u
n
t
 
a
 
f
i
l
e
 
s
y
s
t
e
m
,
 
a
n
d
 
e
v
e
r
y
 
i
m
a
g
e


 
 
 
h
e
r
e
 
t
h
a
t
 
a
 
f
i
l
e
 
s
y
s
t
e
m
 
m
a
d
e
 
h
a
s
 
n
e
v
e
r
 
c
o
n
s
u
m
e
d
 
a
n
 
e
n
t
r
y
.
 
 
I
t
 
i
s
 
t
h
e
 
t
h
i
n
g
 
t
h
a
t


 
 
 
w
o
u
l
d
 
s
e
t
t
l
e
 
t
h
e
 
f
r
e
e
 
l
i
s
t
'
s
 
o
w
n
 
t
r
a
n
s
i
t
i
o
n
 
q
u
e
s
t
i
o
n
 
—
 
s
e
e


 
 
 
[
t
h
a
t
 
s
e
c
t
i
o
n
]
(
#
t
h
e
-
a
g
f
l
-
-
o
r
d
i
n
a
r
y
-
f
r
e
e
-
s
p
a
c
e
-
q
u
e
s
t
i
o
n
-
i
s
-
u
n
m
e
a
s
u
r
e
d
)
 
—
 
a
n
d


 
 
 
u
n
t
i
l
 
i
t
 
e
x
i
s
t
s
,
 
a
 
c
o
u
p
l
e
 
o
f
 
q
u
e
s
t
i
o
n
s
 
b
e
l
o
w
 
a
r
e
 
b
l
o
c
k
e
d
 
o
n
 
e
v
i
d
e
n
c
e
 
r
a
t
h
e
r
 
t
h
a
n


 
 
 
o
n
 
a
n
y
t
h
i
n
g
 
t
o
 
w
r
i
t
e
.




1
.
 
*
*
L
i
n
k
i
n
g
 
a
 
n
o
d
e
 
i
n
.
*
*
 
 
A
 
b
l
o
c
k
 
t
a
k
e
n
 
f
o
r
 
a
 
n
o
d
e
 
i
s
 
c
h
a
r
g
e
d
 
f
o
r
 
t
h
e
 
g
r
o
u
p


 
 
 
b
e
f
o
r
e
 
i
t
 
b
e
l
o
n
g
s
 
t
o
 
a
n
y
 
t
r
e
e
,
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
r
e
f
u
s
e
s
 
t
h
a
t
 
s
t
a
t
e
:




 
 
 
`
`
`
t
e
x
t


 
 
 
a
g
f
_
b
t
r
e
e
b
l
k
s
 
1
,
 
c
o
u
n
t
e
d
 
0
 
i
n
 
a
g
 
0


 
 
 
s
b
_
f
d
b
l
o
c
k
s
 
9
0
6
2
4
,
 
c
o
u
n
t
e
d
 
9
0
6
2
3


 
 
 
`
`
`




 
 
 
I
t
 
i
s
 
t
h
e
 
s
t
a
t
e
 
b
e
t
w
e
e
n
 
t
a
k
i
n
g
 
a
 
n
o
d
e
 
a
n
d
 
l
i
n
k
i
n
g
 
i
t
,
 
w
h
i
c
h
 
a
 
s
p
l
i
t
 
p
a
s
s
e
s


 
 
 
t
h
r
o
u
g
h
,
 
s
o
 
t
h
e
 
f
i
x
 
i
s
 
n
o
t
 
t
o
 
s
t
o
p
 
c
h
a
r
g
i
n
g
 
b
u
t
 
t
o
 
f
i
n
i
s
h
 
t
h
e
 
o
p
e
r
a
t
i
o
n
:
 
t
h
e


 
 
 
c
h
a
r
g
e
 
a
n
d
 
t
h
e
 
l
i
n
k
 
h
a
v
e
 
t
o
 
b
e
 
i
n
 
t
h
e
 
s
a
m
e
 
t
r
a
n
s
a
c
t
i
o
n
,
 
w
h
i
c
h
 
t
h
e
y
 
a
r
e
,
 
a
n
d


 
 
 
t
h
e
 
o
p
e
r
a
t
i
o
n
 
h
a
s
 
t
o
 
b
e
 
f
i
n
i
s
h
e
d
,
 
w
h
i
c
h
 
n
o
t
h
i
n
g
 
c
a
n
 
y
e
t
 
d
o
.


2
.
 
*
*
A
n
 
e
m
p
t
i
e
d
 
w
i
n
d
o
w
:
 
r
e
s
e
t
 
o
r
 
w
r
a
p
.
*
*
 
 
X
F
S
'
s
 
o
w
n
 
g
r
o
u
p
 
r
e
b
u
i
l
d
 
r
e
s
e
t
 
b
o
t
h


 
 
 
w
i
n
d
o
w
s
 
t
o
 
s
t
a
r
t
 
a
t
 
z
e
r
o
,
 
b
u
t
 
t
h
e
 
h
a
n
d
-
b
u
i
l
t
 
i
m
a
g
e
'
s
 
g
r
o
u
p
s
 
1
 
a
n
d
 
3
 
s
i
t
 
a
t
 
8
5


 
 
 
a
n
d
 
2
6
,
 
w
h
i
c
h
 
i
s
 
e
v
i
d
e
n
c
e
 
t
h
a
t
 
a
 
w
i
n
d
o
w
 
d
o
e
s
 
n
o
t
 
a
l
w
a
y
s
 
g
o
 
b
a
c
k
 
t
o
 
z
e
r
o
.
 
 
T
h
e


 
 
 
c
o
d
e
 
r
e
s
e
t
s
,
 
w
h
i
c
h
 
k
e
e
p
s
 
t
h
e
 
m
o
d
e
l
 
c
l
o
s
e
d
 
—
 
a
 
w
i
n
d
o
w
 
t
h
a
t
 
a
d
v
a
n
c
e
d
 
p
a
s
t
 
t
h
e
 
e
n
d


 
 
 
o
f
 
t
h
e
 
a
r
r
a
y
 
w
o
u
l
d
 
n
a
m
e
 
a
 
s
l
o
t
 
t
h
a
t
 
i
s
 
n
o
t
 
t
h
e
r
e
 
—
 
a
n
d
 
t
h
e
 
c
h
o
i
c
e
 
i
s
 
m
a
d
e
 
i
n


 
 
 
o
n
e
 
p
l
a
c
e
 
s
o
 
i
t
 
c
a
n
 
b
e
 
c
h
a
n
g
e
d
 
w
h
e
n
 
i
t
 
i
s
 
e
s
t
a
b
l
i
s
h
e
d
.


3
.
 
*
*
R
o
o
t
 
c
o
l
l
a
p
s
e
.
*
*
 
 
L
e
a
f
 
m
e
r
g
i
n
g
 
w
o
r
k
s
 
—
 
s
e
e
 
b
e
l
o
w
 
—
 
b
u
t
 
a
 
r
o
o
t
 
l
e
f
t
 
w
i
t
h
 
a


 
 
 
s
i
n
g
l
e
 
c
h
i
l
d
 
i
s
 
n
o
t
 
c
o
l
l
a
p
s
e
d
 
i
n
t
o
 
i
t
.
 
 
I
t
 
c
a
n
n
o
t
 
b
e
 
r
e
a
c
h
e
d
 
y
e
t
,
 
w
h
i
c
h
 
i
s
 
w
h
y


 
 
 
i
t
 
i
s
 
n
o
t
 
b
u
i
l
t
:
 
a
 
p
a
r
e
n
t
 
a
l
w
a
y
s
 
h
o
l
d
s
 
a
t
 
l
e
a
s
t
 
t
w
o
 
c
h
i
l
d
r
e
n
,
 
t
h
e
 
m
e
r
g
e
 
b
r
a
n
c
h


 
 
 
r
e
f
u
s
e
s
 
t
o
 
t
a
k
e
 
t
h
e
 
l
a
s
t
 
o
n
e
,
 
a
n
d
 
n
o
t
h
i
n
g
 
e
l
s
e
 
r
e
m
o
v
e
s
 
a
 
n
o
d
e
,
 
s
o
 
n
o
 
i
n
t
e
r
i
o
r


 
 
 
n
o
d
e
 
c
a
n
 
b
e
 
e
m
p
t
i
e
d
.
 
 
I
t
 
b
e
c
o
m
e
s
 
r
e
a
c
h
a
b
l
e
 
t
h
e
 
m
o
m
e
n
t
 
s
o
m
e
t
h
i
n
g
 
c
a
n
 
e
m
p
t
y
 
a


 
 
 
p
a
r
e
n
t
,
 
a
n
d
 
i
t
 
s
h
o
u
l
d
 
b
e
 
w
r
i
t
t
e
n
 
t
h
e
n
,
 
a
g
a
i
n
s
t
 
a
 
t
e
s
t
 
t
h
a
t
 
g
e
t
s
 
t
h
e
r
e
,
 
r
a
t
h
e
r


 
 
 
t
h
a
n
 
s
p
e
c
u
l
a
t
i
v
e
l
y
.




#
#
#
 
N
o
t
 
s
t
a
r
t
e
d




|
 
A
r
e
a
 
|


|
:
-
-
-
-
-
|


|
 
R
o
o
t
 
c
o
l
l
a
p
s
e
 
(
a
n
 
i
n
t
e
r
i
o
r
 
r
o
o
t
 
l
e
f
t
 
w
i
t
h
 
o
n
e
 
c
h
i
l
d
)
 
|


|
 
B
M
B
T
 
g
r
o
w
t
h
:
 
a
 
d
a
t
a
 
f
o
r
k
 
t
h
a
t
 
h
a
s
 
t
o
 
b
e
c
o
m
e
 
a
 
b
-
t
r
e
e
 
|


|
 
`
c
r
e
a
t
e
`
,
 
`
u
n
l
i
n
k
`
,
 
d
i
r
e
c
t
o
r
y
 
a
n
d
 
n
a
m
e
s
p
a
c
e
 
m
u
t
a
t
i
o
n
 
|


|
 
J
o
u
r
n
a
l
,
 
l
o
g
 
r
e
c
o
v
e
r
y
,
 
c
r
a
s
h
 
s
a
f
e
t
y
 
|


|
 
R
e
a
l
-
t
i
m
e
 
d
e
v
i
c
e
 
a
l
l
o
c
a
t
i
o
n
 
|


|
 
R
e
v
e
r
s
e
 
m
a
p
p
i
n
g
 
a
n
d
 
r
e
f
e
r
e
n
c
e
 
c
o
u
n
t
 
t
r
e
e
s
 
|


|
 
S
p
a
r
s
e
-
f
i
l
e
 
h
o
l
e
 
p
u
n
c
h
i
n
g
,
 
`
f
a
l
l
o
c
a
t
e
`
 
|




-
-
-




#
#
 
H
o
w
 
s
t
r
o
n
g
 
e
a
c
h
 
m
e
a
s
u
r
e
m
e
n
t
 
i
s
,
 
a
n
d
 
h
o
w
 
t
o
 
r
e
a
d
 
o
n
e




T
h
e
 
c
l
a
i
m
 
t
h
a
t
 
s
t
a
r
t
e
d
 
t
h
i
s
 
s
e
c
t
i
o
n
'
s
 
a
u
d
i
t
 
w
a
s
 
w
r
o
n
g
 
t
w
i
c
e
:
 
a
 
f
r
e
e
 
i
n
o
d
e
'
s
 
s
l
o
t


w
a
s
 
s
a
i
d
 
t
o
 
b
e
 
a
l
l
 
z
e
r
o
e
s
 
b
e
c
a
u
s
e
 
a
 
h
a
n
d
-
w
r
i
t
t
e
n
 
p
a
r
s
e
r
 
r
e
a
d
 
i
t
 
t
h
a
t
 
w
a
y
,
 
a
n
d
 
t
h
e


p
a
r
s
e
r
 
h
a
d
 
r
e
a
d
 
*
o
u
t
 
o
f
 
b
o
u
n
d
s
*
.
 
 
T
h
e
 
n
u
m
b
e
r
s
 
i
t
 
p
r
o
d
u
c
e
d
 
w
e
r
e
 
p
l
a
u
s
i
b
l
e
,
 
t
h
e
y


a
g
r
e
e
d
 
w
i
t
h
 
a
 
f
a
m
i
l
i
a
r
 
s
t
r
u
c
t
u
r
e
,
 
a
n
d
 
n
o
t
h
i
n
g
 
c
a
u
g
h
t
 
i
t
 
f
o
r
 
a
 
l
o
n
g
 
t
i
m
e
.




S
o
:
 
*
*
a
 
h
a
n
d
 
p
a
r
s
e
r
 
i
s
 
a
 
m
e
a
s
u
r
e
m
e
n
t
 
i
n
s
t
r
u
m
e
n
t
,
 
n
o
t
 
a
 
s
p
e
c
i
f
i
c
a
t
i
o
n
.
*
*
 
 
B
e
i
n
g


i
n
d
e
p
e
n
d
e
n
t
 
o
f
 
`
x
f
u
s
e
`
 
d
o
e
s
 
n
o
t
 
m
a
k
e
 
i
t
 
a
n
 
o
r
a
c
l
e
 
—
 
i
t
 
m
a
k
e
s
 
i
t
 
a
 
s
e
c
o
n
d
 
o
p
i
n
i
o
n


f
r
o
m
 
t
h
e
 
s
a
m
e
 
k
i
n
d
 
o
f
 
r
e
a
d
e
r
,
 
a
n
d
 
i
t
 
c
a
n
 
b
e
 
w
r
o
n
g
 
i
n
 
t
h
e
 
s
a
m
e
 
p
l
a
u
s
i
b
l
e
 
w
a
y
.




E
v
e
r
y
 
c
l
a
i
m
 
b
e
l
o
w
 
t
h
e
r
e
f
o
r
e
 
c
a
r
r
i
e
s
 
a
 
l
e
v
e
l
,
 
a
n
d
 
t
h
e
 
l
e
v
e
l
s
 
m
e
a
n
 
t
h
i
s
:




|
 
l
e
v
e
l
 
|
 
w
h
a
t
 
i
t
 
i
s
 
|
 
e
x
a
m
p
l
e
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
|


|
 
*
*
t
o
o
l
*
*
 
|
 
a
 
n
a
t
i
v
e
 
t
o
o
l
 
p
r
i
n
t
e
d
 
i
t
,
 
o
r
 
i
t
s
 
a
n
s
w
e
r
 
t
o
 
a
 
c
o
n
t
r
o
l
l
e
d
 
e
x
p
e
r
i
m
e
n
t
 
|
 
`
x
f
s
_
d
b
`
'
s
 
`
b
l
o
c
k
g
e
t
`
 
w
a
l
k
;
 
r
e
p
a
i
r
'
s
 
`
b
a
d
 
n
e
x
t
_
u
n
l
i
n
k
e
d
 
0
x
0
`
 
|


|
 
*
*
c
o
r
r
o
b
o
r
a
t
e
d
*
*
 
|
 
t
h
i
s
 
c
o
d
e
'
s
 
r
e
a
d
e
r
 
a
n
d
 
a
 
n
a
t
i
v
e
 
t
o
o
l
 
a
g
r
e
e
 
a
b
o
u
t
 
t
h
e
 
s
a
m
e
 
f
i
e
l
d
 
|
 
`
x
f
s
_
d
b
`
'
s
 
i
n
o
b
t
 
d
u
m
p
 
a
n
d
 
`
c
h
u
n
k
s
_
i
n
_
o
r
d
e
r
`
 
|


|
 
*
*
d
e
r
i
v
e
d
*
*
 
|
 
a
r
i
t
h
m
e
t
i
c
 
o
v
e
r
 
m
e
a
s
u
r
e
d
 
q
u
a
n
t
i
t
i
e
s
 
|
 
t
h
e
 
n
i
n
e
 
e
x
t
e
n
t
s
 
a
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
 
h
o
l
d
s
 
|


|
 
*
*
i
n
f
e
r
e
n
c
e
*
*
 
|
 
a
 
c
o
n
c
l
u
s
i
o
n
 
f
r
o
m
 
d
o
c
u
m
e
n
t
e
d
 
b
e
h
a
v
i
o
u
r
,
 
w
i
t
h
 
n
o
 
e
x
p
e
r
i
m
e
n
t
 
b
e
h
i
n
d
 
i
t
 
|
 
t
h
a
t
 
a
n
 
u
n
m
e
a
s
u
r
e
d
 
t
r
a
n
s
i
t
i
o
n
 
b
e
h
a
v
e
s
 
l
i
k
e
 
a
 
m
e
a
s
u
r
e
d
 
o
n
e
 
|


|
 
*
*
i
n
s
t
r
u
m
e
n
t
*
*
 
|
 
a
 
h
a
n
d
 
p
a
r
s
e
r
,
 
f
o
r
 
e
x
p
l
o
r
i
n
g
 
|
 
t
h
e
 
t
h
r
o
w
a
w
a
y
 
w
a
l
k
e
r
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
u
s
e
d
 
t
o
 
c
i
t
e
 
|




#
#
#
 
T
h
e
 
a
u
d
i
t




E
a
c
h
 
n
u
m
e
r
i
c
a
l
 
c
l
a
i
m
 
i
n
 
t
h
i
s
 
d
o
c
u
m
e
n
t
,
 
w
h
a
t
 
i
t
 
r
e
s
t
s
 
o
n
,
 
a
n
d
 
w
h
e
r
e
 
i
t
 
n
o
w
 
s
t
a
n
d
s
.




|
 
c
l
a
i
m
 
|
 
l
e
v
e
l
 
|
 
i
n
d
e
p
e
n
d
e
n
t
 
c
o
n
f
i
r
m
a
t
i
o
n
 
|
 
s
t
a
t
u
s
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|


|
 
`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
i
s
 
t
h
e
 
b
n
o
 
t
r
e
e
'
s
 
r
e
c
o
r
d
 
t
o
t
a
l
 
|
 
t
o
o
l
 
|
 
`
x
f
s
_
d
b
`
'
s
 
`
b
l
o
c
k
g
e
t
`
 
w
a
l
k
 
c
o
u
n
t
s
 
9
0
2
7
7
 
f
r
e
e
 
b
l
o
c
k
s
 
i
n
 
t
h
e
 
b
y
-
b
l
o
c
k
 
t
r
e
e
,
 
a
n
d
 
t
h
e
 
h
e
a
d
e
r
s
 
s
u
m
 
t
o
 
9
0
2
7
7
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
b
o
t
h
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
h
o
l
d
 
t
h
e
 
s
a
m
e
 
f
r
e
e
 
b
l
o
c
k
s
 
|
 
t
o
o
l
 
|
 
t
h
e
 
s
a
m
e
 
w
a
l
k
 
c
o
u
n
t
s
 
9
0
2
7
7
 
i
n
 
e
a
c
h
 
o
f
 
`
f
r
e
e
1
`
 
a
n
d
 
`
f
r
e
e
2
`
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
i
s
 
t
h
e
 
t
r
e
e
s
'
 
b
l
o
c
k
s
 
l
e
s
s
 
t
w
o
 
r
o
o
t
s
 
|
 
t
o
o
l
 
|
 
t
h
e
 
w
a
l
k
 
f
i
n
d
s
 
3
3
3
 
t
r
e
e
 
n
o
d
e
s
 
a
n
d
 
t
h
e
 
h
e
a
d
e
r
s
 
c
h
a
r
g
e
 
3
2
5
,
 
w
h
i
c
h
 
i
s
 
3
3
3
 
−
 
2
 
r
o
o
t
s
 
i
n
 
e
a
c
h
 
o
f
 
4
 
g
r
o
u
p
s
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
`
s
b
_
f
d
b
l
o
c
k
s
`
 
i
s
 
t
h
e
 
t
h
r
e
e
 
t
e
r
m
s
 
s
u
m
m
e
d
 
|
 
t
o
o
l
 
|
 
9
0
2
7
7
 
+
 
3
2
5
 
+
 
2
2
 
=
 
9
0
6
2
4
,
 
a
l
l
 
f
r
o
m
 
`
x
f
s
_
d
b
`
,
 
o
n
 
a
l
l
 
t
h
r
e
e
 
i
m
a
g
e
s
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
A
G
F
L
 
b
l
o
c
k
s
 
a
r
e
 
n
o
t
 
i
n
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
|
 
c
o
r
r
o
b
o
r
a
t
e
d
 
|
 
t
h
e
 
w
a
l
k
'
s
 
`
f
r
e
e
l
i
s
t
`
 
l
a
b
e
l
 
a
n
d
 
i
t
s
 
`
f
r
e
e
1
`
/
`
f
r
e
e
2
`
 
l
a
b
e
l
s
 
a
r
e
 
d
i
s
j
o
i
n
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
t
h
e
 
l
i
v
e
 
w
i
n
d
o
w
 
i
s
 
a
 
s
l
i
c
e
 
t
h
a
t
 
d
o
e
s
 
n
o
t
 
s
t
a
r
t
 
a
t
 
z
e
r
o
 
|
 
t
o
o
l
 
|
 
`
x
f
s
_
d
b
`
 
p
r
i
n
t
s
 
`
f
l
f
i
r
s
t
 
=
 
8
5
`
 
f
o
r
 
`
x
f
s
v
4
.
i
m
g
`
 
g
r
o
u
p
 
1
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
n
o
n
-
r
o
o
t
 
l
e
a
f
 
n
e
e
d
s
 
3
1
 
r
e
c
o
r
d
s
,
 
a
 
r
o
o
t
 
l
e
a
f
 
n
o
n
e
 
|
 
t
o
o
l
 
|
 
r
e
p
a
i
r
'
s
 
`
b
a
d
 
b
t
r
e
e
 
n
r
e
c
s
 
(
3
0
,
 
m
i
n
=
3
1
,
 
m
a
x
=
6
2
)
`
,
 
a
n
d
 
s
i
l
e
n
c
e
 
f
o
r
 
a
 
r
o
o
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
`
n
e
x
t
_
u
n
l
i
n
k
e
d
`
 
i
s
 
a
t
 
o
f
f
s
e
t
 
9
6
 
a
n
d
 
h
o
l
d
s
 
`
0
x
f
f
f
f
f
f
f
f
`
 
|
 
t
o
o
l
 
|
 
r
e
p
a
i
r
 
n
a
m
e
s
 
t
h
e
 
f
i
e
l
d
;
 
i
t
s
 
o
f
f
s
e
t
 
f
o
u
n
d
 
b
y
 
w
r
i
t
i
n
g
 
a
 
v
a
l
u
e
 
a
t
 
e
a
c
h
 
o
f
f
s
e
t
 
a
n
d
 
a
s
k
i
n
g
 
w
h
i
c
h
 
i
t
 
r
e
a
d
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
`
s
t
a
r
t
i
n
o
`
 
i
s
 
c
o
u
n
t
e
d
 
f
r
o
m
 
t
h
e
 
g
r
o
u
p
 
|
 
t
o
o
l
 
|
 
r
e
p
a
i
r
 
a
c
c
e
p
t
s
 
b
e
l
o
w
 
`
1
5
3
6
0
0
 
×
 
2
`
 
a
n
d
 
r
e
f
u
s
e
s
 
a
t
 
o
r
 
a
b
o
v
e
 
i
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
 
h
o
l
d
s
 
n
i
n
e
 
e
x
t
e
n
t
s
 
|
 
d
e
r
i
v
e
d
 
+
 
t
o
o
l
 
|
 
`
(
2
5
6
 
−
 
1
0
0
)
 
/
 
1
6
`
,
 
a
n
d
 
n
i
n
e
 
s
p
a
r
s
e
 
w
r
i
t
e
s
 
s
u
c
c
e
e
d
 
b
e
f
o
r
e
 
t
h
e
 
t
e
n
t
h
 
i
s
 
r
e
f
u
s
e
d
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
t
h
e
 
f
r
e
e
 
s
l
o
t
s
 
o
f
 
a
 
n
e
w
 
c
h
u
n
k
:
 
m
a
g
i
c
,
 
v
e
r
s
i
o
n
,
 
`
n
e
x
t
_
u
n
l
i
n
k
e
d
`
 
|
 
t
o
o
l
 
|
 
r
e
p
a
i
r
'
s
 
c
o
m
p
l
a
i
n
t
s
,
 
i
t
e
m
 
b
y
 
i
t
e
m
 
|
 
*
*
r
e
p
a
i
r
-
v
a
l
i
d
a
t
e
d
 
o
n
l
y
*
*
 
|


|
 
—
 
t
h
e
 
*
v
e
r
s
i
o
n
*
 
X
F
S
 
p
i
c
k
s
 
f
o
r
 
a
 
n
e
w
 
c
h
u
n
k
'
s
 
s
l
o
t
s
 
|
 
i
n
f
e
r
e
n
c
e
 
|
 
n
o
n
e
:
 
n
o
 
i
m
a
g
e
 
h
e
r
e
 
h
a
s
 
a
 
c
h
u
n
k
 
X
F
S
 
c
r
e
a
t
e
d
 
|
 
*
*
u
n
m
e
a
s
u
r
e
d
*
*
 
|


|
 
A
G
F
L
 
→
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
,
 
w
h
e
n
 
t
h
e
 
l
i
s
t
 
i
s
 
f
u
l
l
 
|
 
t
o
o
l
 
|
 
a
 
f
u
l
l
 
l
i
s
t
 
h
a
n
d
e
d
 
t
o
 
`
x
f
s
_
r
e
p
a
i
r
`
 
c
o
m
e
s
 
b
a
c
k
 
w
i
t
h
 
*
*
e
v
e
r
y
*
*
 
o
n
e
 
o
f
 
i
t
s
 
4
2
 
b
l
o
c
k
s
 
a
s
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
,
 
w
i
t
h
 
a
 
c
a
v
e
a
t
 
a
b
o
u
t
 
w
h
o
 
d
i
d
 
i
t
 
|


|
 
a
 
b
-
m
a
p
 
l
e
a
f
'
s
 
m
a
g
i
c
,
 
l
e
v
e
l
,
 
r
e
c
o
r
d
 
c
o
u
n
t
 
a
n
d
 
s
i
b
l
i
n
g
 
f
i
e
l
d
s
 
|
 
t
o
o
l
 
|
 
`
B
M
A
P
`
 
(
`
0
x
4
2
4
d
4
1
5
0
`
)
 
a
t
 
0
,
 
l
e
v
e
l
 
a
t
 
4
,
 
c
o
u
n
t
 
a
t
 
6
,
 
s
i
b
l
i
n
g
s
 
a
t
 
8
 
a
n
d
 
1
2
,
 
t
h
e
 
s
a
m
e
 
i
n
 
t
h
r
e
e
 
l
e
a
v
e
s
 
`
x
f
s
_
d
b
`
 
c
a
n
 
n
a
m
e
 
t
h
e
 
e
x
t
e
n
t
s
 
o
f
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
e
x
t
e
n
t
'
s
 
s
t
a
r
t
 
i
s
 
a
t
 
o
f
f
s
e
t
 
2
4
,
 
i
n
 
b
y
t
e
s
 
|
 
t
o
o
l
 
|
 
w
r
i
t
i
n
g
 
7
7
7
7
7
7
 
t
h
e
r
e
 
p
r
o
d
u
c
e
d
 
`
o
f
f
s
e
t
 
1
5
1
9
`
,
 
a
n
d
 
7
7
7
7
7
7
 
/
 
5
1
2
 
i
s
 
1
5
1
9
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
o
f
f
s
e
t
s
 
1
2
 
a
n
d
 
4
4
 
a
r
e
 
n
o
t
 
r
e
c
o
r
d
 
f
i
e
l
d
s
 
|
 
t
o
o
l
 
|
 
p
e
r
t
u
r
b
i
n
g
 
e
i
t
h
e
r
 
p
r
o
d
u
c
e
d
 
n
o
 
e
x
t
e
n
t
 
c
o
m
p
l
a
i
n
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
t
h
e
 
"
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
"
 
r
e
p
a
i
r
 
r
e
p
o
r
t
s
 
i
s
 
c
o
m
p
o
s
i
t
e
 
|
 
t
o
o
l
 
|
 
t
h
e
 
l
o
w
 
h
a
l
f
 
o
f
 
o
f
f
s
e
t
 
3
2
 
c
h
a
n
g
e
s
 
i
t
 
t
o
 
`
1
5
9
2
8
8
8
4
5
6
`
 
a
n
d
 
t
h
e
 
h
i
g
h
 
h
a
l
f
 
t
o
 
`
4
9
1
5
2
`
;
 
n
e
i
t
h
e
r
 
i
s
 
a
n
y
 
b
y
t
e
 
r
a
n
g
e
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
e
x
t
e
n
t
'
s
 
*
l
e
n
g
t
h
*
 
i
s
 
a
t
 
o
f
f
s
e
t
 
3
2
 
|
 
t
o
o
l
 
|
 
p
a
t
c
h
i
n
g
 
i
t
 
m
o
v
e
s
 
r
e
p
a
i
r
'
s
 
r
e
p
o
r
t
e
d
 
s
t
a
r
t
 
a
n
d
 
e
n
d
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
a
 
l
e
n
g
t
h
 
d
o
e
s
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
o
f
f
s
e
t
s
 
1
6
 
a
n
d
 
4
4
 
h
o
l
d
 
n
o
 
v
a
l
i
d
a
t
e
d
 
f
i
e
l
d
 
|
 
t
o
o
l
 
|
 
a
n
 
u
n
m
i
s
t
a
k
a
b
l
e
 
v
a
l
u
e
 
a
t
 
e
i
t
h
e
r
 
p
r
o
d
u
c
e
s
 
n
o
 
e
x
t
e
n
t
 
c
o
m
p
l
a
i
n
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
r
e
c
o
r
d
'
s
 
f
i
e
l
d
s
 
|
 
t
o
o
l
 
|
 
`
x
f
s
_
r
e
p
a
i
r
`
 
p
r
i
n
t
s
 
t
h
e
m
:
 
`
b
m
a
p
 
r
e
c
 
o
u
t
 
o
f
 
o
r
d
e
r
 
.
.
.
 
[
o
 
s
 
c
]
`
 
—
 
o
f
f
s
e
t
,
 
s
t
a
r
t
 
b
l
o
c
k
,
 
c
o
u
n
t
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
r
e
c
o
r
d
'
s
 
s
i
z
e
 
a
n
d
 
f
i
e
l
d
 
o
f
f
s
e
t
s
 
|
 
t
o
o
l
 
|
 
1
6
 
b
y
t
e
s
;
 
`
o
`
 
a
t
 
2
4
+
1
6
n
,
 
`
s
`
 
a
t
 
2
8
+
1
6
n
,
 
`
c
`
 
a
t
 
3
2
+
1
6
n
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
r
e
c
o
r
d
'
s
 
s
h
a
p
e
:
 
1
6
 
b
y
t
e
s
,
 
f
o
u
r
 
4
-
b
y
t
e
 
w
o
r
d
s
,
 
f
r
o
m
 
o
f
f
s
e
t
 
2
4
 
|
 
t
o
o
l
 
|
 
p
e
r
t
u
r
b
i
n
g
 
t
h
e
 
w
o
r
d
 
a
t
 
2
4
 
c
h
a
n
g
e
s
 
e
n
t
r
y
 
0
,
 
a
t
 
4
0
 
c
h
a
n
g
e
s
 
e
n
t
r
y
 
1
,
 
t
h
r
o
u
g
h
 
e
n
t
r
y
 
4
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
e
x
t
e
n
t
'
s
 
f
i
l
e
 
o
f
f
s
e
t
 
i
s
 
t
h
e
 
r
e
c
o
r
d
'
s
 
*
*
s
e
c
o
n
d
*
*
 
w
o
r
d
,
 
i
n
 
*
*
b
y
t
e
s
*
*
 
|
 
t
o
o
l
 
|
 
i
t
 
r
e
a
d
s
 
0
,
 
5
1
2
,
 
1
0
2
4
,
 
1
5
3
6
 
—
 
t
h
e
 
o
f
f
s
e
t
s
 
0
,
 
1
,
 
2
,
 
3
 
t
h
a
t
 
r
e
p
a
i
r
 
*
p
r
i
n
t
s
*
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
r
e
c
o
r
d
'
s
 
f
o
u
r
t
h
 
w
o
r
d
 
i
s
 
`
(
e
n
t
r
y
 
<
<
 
1
6
)
 
\
|
 
l
e
n
g
t
h
 
i
n
 
b
l
o
c
k
s
`
,
 
n
o
t
 
a
 
b
l
o
c
k
 
|
 
t
o
o
l
 
|
 
i
t
s
 
l
o
w
 
h
a
l
f
 
i
s
 
1
 
i
n
 
a
l
l
 
6
4
 
e
n
t
r
i
e
s
 
a
n
d
 
i
t
s
 
h
i
g
h
 
h
a
l
f
 
c
o
u
n
t
s
 
0
.
.
6
3
 
a
c
r
o
s
s
 
t
h
r
e
e
 
l
e
a
v
e
s
 
w
i
t
h
o
u
t
 
a
 
b
r
e
a
k
 
|
 
*
*
c
o
n
f
i
r
m
e
d
*
*
 
|


|
 
a
 
b
-
m
a
p
 
e
x
t
e
n
t
'
s
 
d
a
t
a
 
b
l
o
c
k
:
 
w
h
e
r
e
 
i
t
 
i
s
 
|
 
—
 
|
 
*
*
n
o
t
 
i
n
 
t
h
e
 
r
e
c
o
r
d
*
*
;
 
a
n
d
 
r
e
p
a
i
r
 
m
a
y
 
b
e
 
r
e
c
o
n
s
t
r
u
c
t
i
n
g
 
r
a
t
h
e
r
 
t
h
a
n
 
r
e
a
d
i
n
g
 
i
t
 
|
 
*
*
u
n
m
e
a
s
u
r
e
d
*
*
,
 
w
i
t
h
 
a
 
o
n
e
-
p
e
r
t
u
r
b
a
t
i
o
n
 
t
e
s
t
 
n
a
m
e
d
 
|


|
 
r
o
o
t
 
c
o
l
l
a
p
s
e
 
|
 
—
 
|
 
u
n
r
e
a
c
h
a
b
l
e
,
 
s
o
 
n
o
t
h
i
n
g
 
t
o
 
c
o
n
f
i
r
m
 
|
 
*
*
n
o
t
 
w
r
i
t
t
e
n
*
*
 
|




*
*
T
w
o
 
r
o
w
s
 
a
r
e
 
s
t
i
l
l
 
o
p
e
n
*
*
,
 
a
n
d
 
b
o
t
h
 
a
r
e
 
t
h
e
 
d
i
f
f
e
r
e
n
c
e
 
b
e
t
w
e
e
n
 
"
t
h
i
s
 
i
s
 
h
o
w
 
i
t


w
o
r
k
s
"
 
a
n
d
 
"
t
h
i
s
 
i
s
 
h
o
w
 
i
t
 
h
a
p
p
e
n
s
 
t
o
 
w
o
r
k
 
h
e
r
e
"
.
 
 
E
v
e
r
y
t
h
i
n
g
 
e
l
s
e
 
a
b
o
v
e
 
i
s


`
c
o
n
f
i
r
m
e
d
`
 
o
r
 
`
r
e
p
a
i
r
-
v
a
l
i
d
a
t
e
d
`
,
 
a
n
d
 
t
h
e
 
m
e
t
h
o
d
 
t
h
a
t
 
g
o
t
 
i
t
 
t
h
e
r
e
 
i
s


[
H
o
w
 
a
 
f
o
r
m
a
t
 
f
a
c
t
 
i
s
 
e
s
t
a
b
l
i
s
h
e
d
 
h
e
r
e
]
(
#
h
o
w
-
a
-
f
o
r
m
a
t
-
f
a
c
t
-
i
s
-
e
s
t
a
b
l
i
s
h
e
d
-
h
e
r
e
)


—
 
r
e
a
d
 
t
h
a
t
 
b
e
f
o
r
e
 
a
d
d
i
n
g
 
a
 
r
o
w
,
 
b
e
c
a
u
s
e
 
i
t
 
i
s
 
w
h
a
t
 
k
e
e
p
s
 
a
 
r
o
w
 
f
r
o
m
 
b
e
i
n
g
 
a


g
u
e
s
s
.




*
 
*
*
T
h
e
 
f
r
e
e
 
s
l
o
t
s
 
o
f
 
a
 
n
e
w
 
c
h
u
n
k
 
a
r
e
 
r
e
p
a
i
r
-
v
a
l
i
d
a
t
e
d
,
 
n
o
t
 
c
o
n
f
i
r
m
e
d
.
*
*
 
 
R
e
p
a
i
r


 
 
s
a
y
s
 
w
h
a
t
 
i
t
 
w
i
l
l
 
a
c
c
e
p
t
,
 
a
n
d
 
t
h
e
 
l
a
y
o
u
t
 
h
e
r
e
 
i
s
 
w
h
a
t
 
i
t
 
a
c
c
e
p
t
s
.
 
 
T
h
a
t
 
i
s
 
n
o
t


 
 
t
h
e
 
s
a
m
e
 
a
s
 
k
n
o
w
i
n
g
 
i
t
 
i
s
 
w
h
a
t
 
X
F
S
 
w
r
i
t
e
s
,
 
b
e
c
a
u
s
e
 
n
o
 
c
h
u
n
k
 
i
n
 
a
n
y
 
i
m
a
g
e
 
i
n


 
 
t
h
i
s
 
r
e
p
o
s
i
t
o
r
y
 
w
a
s
 
c
r
e
a
t
e
d
 
b
y
 
a
n
 
o
p
e
r
a
t
i
o
n
 
a
n
y
o
n
e
 
h
e
r
e
 
c
a
n
 
w
a
t
c
h
.




 
 
N
a
r
r
o
w
e
d
 
a
s
 
f
a
r
 
a
s
 
i
t
 
c
a
n
 
b
e
 
f
r
o
m
 
h
e
r
e
,
 
t
h
o
u
g
h
.
 
 
T
h
e
 
v
e
r
s
i
o
n
 
i
s
 
n
o
 
l
o
n
g
e
r
 
a


 
 
f
r
e
e
 
c
h
o
i
c
e
:
 
p
a
t
c
h
i
n
g
 
a
 
f
r
e
s
h
 
s
l
o
t
 
t
o
 
v
e
r
s
i
o
n
 
3
 
a
n
d
 
a
s
k
i
n
g
 
g
e
t
s




 
 
`
`
`
t
e
x
t


 
 
b
a
d
 
v
e
r
s
i
o
n
 
n
u
m
b
e
r
 
0
x
3
 
o
n
 
i
n
o
d
e
 
5
2
4
3
5
2
,
 
w
o
u
l
d
 
r
e
s
e
t
 
v
e
r
s
i
o
n
 
n
u
m
b
e
r


 
 
`
`
`




 
 
s
o
 
*
*
3
 
i
s
 
r
e
f
u
t
e
d
 
f
o
r
 
a
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
*
*
,
 
w
h
i
l
e
 
1
 
a
n
d
 
2
 
a
r
e
 
b
o
t
h
 
a
c
c
e
p
t
e
d
.
 
 
T
h
e


 
 
c
o
d
e
 
w
r
i
t
e
s
 
2
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
e
v
e
r
y
 
i
n
o
d
e
 
i
n
 
t
h
e
s
e
 
i
m
a
g
e
s
 
t
h
a
t
 
a
 
f
i
l
e
 
s
y
s
t
e
m


 
 
*
d
i
d
*
 
w
r
i
t
e
 
r
e
a
d
s
.
 
 
T
h
e
 
5
1
2
-
b
y
t
e
 
h
a
l
f
 
o
f
 
t
h
e
 
c
h
o
i
c
e
 
s
t
a
y
s
 
a
n
 
i
n
f
e
r
e
n
c
e
,
 
b
e
c
a
u
s
e


 
 
n
o
 
i
m
a
g
e
 
h
e
r
e
 
h
a
s
 
o
n
e
 
t
o
 
a
l
l
o
c
a
t
e
 
a
 
c
h
u
n
k
 
i
n
.




 
 
T
h
e
 
s
a
m
e
 
e
x
p
e
r
i
m
e
n
t
 
s
e
p
a
r
a
t
e
s
 
t
w
o
 
t
h
i
n
g
s
 
t
h
a
t
 
w
e
r
e
 
e
a
s
y
 
t
o
 
c
o
n
f
l
a
t
e
:
 
a
 
*
*
f
r
e
e
*
*


 
 
s
l
o
t
'
s
 
a
t
t
r
i
b
u
t
e
 
f
o
r
k
 
r
e
a
d
s
 
`
a
f
o
r
m
a
t
 
=
 
0
`
 
a
n
d
 
r
e
p
a
i
r
 
a
c
c
e
p
t
s
 
i
t
,
 
w
h
i
l
e
 
a
n
 
i
n
o
d
e


 
 
`
a
l
l
o
c
a
t
e
_
i
n
o
`
 
h
a
s
 
h
a
n
d
e
d
 
o
u
t
 
m
u
s
t
 
h
a
v
e
 
i
t
 
s
e
t
 
—
 
`
b
a
d
 
a
t
t
r
i
b
u
t
e
 
f
o
r
m
a
t
 
0
`
 
i
s
 
w
h
a
t


 
 
t
h
a
t
 
w
a
s
 
f
i
x
e
d
 
f
o
r
.
 
 
Z
e
r
o
 
i
s
 
r
i
g
h
t
 
f
o
r
 
o
n
e
 
s
t
a
t
e
 
a
n
d
 
w
r
o
n
g
 
f
o
r
 
t
h
e
 
o
t
h
e
r
,
 
a
n
d
 
b
o
t
h


 
 
a
r
e
 
n
o
w
 
m
e
a
s
u
r
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
a
s
s
u
m
e
d
.




*
 
*
*
"
A
G
F
L
 
→
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
"
 
i
s
 
r
e
a
l
,
 
b
u
t
 
t
h
e
 
a
c
t
o
r
 
i
s
 
t
h
e
 
r
e
c
o
v
e
r
y


 
 
t
o
o
l
.
*
*
 
 
E
v
e
r
y
 
i
m
a
g
e
 
`
m
k
f
s
.
x
f
s
`
 
p
r
o
d
u
c
e
d
 
h
e
r
e
 
h
a
s
 
n
e
v
e
r
 
c
o
n
s
u
m
e
d
 
a
 
l
i
s
t
 
e
n
t
r
y
,


 
 
s
o
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
c
o
u
l
d
 
n
o
t
 
b
e
 
o
b
s
e
r
v
e
d
 
o
n
 
o
n
e
 
-
-
 
a
n
d
 
t
h
e
n
 
i
t
 
w
a
s
 
o
b
s
e
r
v
e
d
 
b
y


 
 
*
m
a
k
i
n
g
*
 
o
n
e
.
 
 
S
t
o
c
k
 
a
 
l
i
s
t
 
u
n
t
i
l
 
i
t
s
 
w
i
n
d
o
w
 
r
e
a
c
h
e
s
 
t
h
e
 
e
n
d
 
o
f
 
t
h
e
 
a
r
r
a
y
,
 
h
a
n
d


 
 
t
h
e
 
i
m
a
g
e
 
t
o
 
`
x
f
s
_
r
e
p
a
i
r
`
,
 
a
n
d
 
s
e
e
:




 
 
`
`
`
t
e
x
t


 
 
b
e
f
o
r
e
:
 
 
t
h
e
 
l
i
s
t
 
i
s
 
(
8
6
,
1
2
7
,
4
2
)
 
h
o
l
d
i
n
g
 
4
2
 
e
n
t
r
i
e
s
 
i
n
 
a
 
1
2
8
-
s
l
o
t
 
a
r
r
a
y


 
 
a
f
t
e
r
:
 
 
 
x
f
s
_
r
e
p
a
i
r
 
r
e
b
u
i
l
t
 
t
h
e
 
l
i
s
t
:
 
w
i
n
d
o
w
 
(
0
,
7
,
8
)
 
w
i
t
h
 
8
 
e
n
t
r
i
e
s


 
 
 
 
 
 
 
 
 
 
 
o
f
 
t
h
e
 
4
2
 
b
l
o
c
k
s
 
t
h
a
t
 
w
e
r
e
 
o
n
 
t
h
e
 
l
i
s
t
:


 
 
 
 
 
 
 
 
 
 
 
 
 
0
 
o
n
 
t
h
e
 
r
e
b
u
i
l
t
 
l
i
s
t
,
 
4
2
 
n
o
w
 
f
r
e
e
 
s
p
a
c
e
,
 
0
 
n
e
i
t
h
e
r


 
 
`
`
`




 
 
*
*
E
v
e
r
y
 
b
l
o
c
k
 
t
h
a
t
 
w
a
s
 
o
n
 
t
h
e
 
f
u
l
l
 
l
i
s
t
 
c
a
m
e
 
b
a
c
k
 
a
s
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
.
*
*
 
 
S
o


 
 
a
 
f
r
e
e
 
l
i
s
t
 
t
h
a
t
 
c
a
n
n
o
t
 
h
o
l
d
 
m
o
r
e
 
d
o
e
s
 
n
o
t
 
k
e
e
p
 
i
t
s
 
b
l
o
c
k
s
:
 
t
h
e
y
 
b
e
c
o
m
e
 
f
r
e
e


 
 
b
l
o
c
k
s
,
 
c
o
u
n
t
e
d
 
i
n
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
a
n
d
 
n
o
w
h
e
r
e
 
e
l
s
e
.
 
 
T
h
a
t
 
i
s
 
w
h
a
t
 
a


 
 
r
e
l
e
a
s
e
d
 
n
o
d
e
 
h
a
s
 
t
o
 
d
o
 
w
h
e
n
 
t
h
e
 
l
i
s
t
 
i
s
 
f
u
l
l
,
 
a
n
d
 
i
t
 
i
s
 
w
h
a
t
 
t
h
e
 
a
l
l
o
c
a
t
o
r
'
s


 
 
f
a
l
l
b
a
c
k
 
b
r
a
n
c
h
 
a
l
r
e
a
d
y
 
d
o
e
s
.




 
 
T
h
e
 
c
a
v
e
a
t
 
i
s
 
a
b
o
u
t
 
*
w
h
o
 
d
i
d
 
i
t
*
.
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
i
s
 
X
F
S
'
s
 
o
w
n
 
c
o
d
e
,
 
s
o
 
t
h
i
s
 
i
s


 
 
X
F
S
'
s
 
a
n
s
w
e
r
 
r
a
t
h
e
r
 
t
h
a
n
 
t
h
i
s
 
p
r
o
j
e
c
t
'
s
 
g
u
e
s
s
 
-
-
 
b
u
t
 
i
t
 
i
s
 
t
h
e
 
*
*
r
e
c
o
v
e
r
y
 
t
o
o
l
*
*


 
 
d
i
s
c
a
r
d
i
n
g
 
a
n
d
 
r
e
b
u
i
l
d
i
n
g
 
a
 
l
i
s
t
,
 
n
o
t
 
t
h
e
 
r
u
n
n
i
n
g
 
f
i
l
e
 
s
y
s
t
e
m
 
d
e
c
i
d
i
n
g
 
w
h
e
r
e
 
t
o


 
 
p
u
t
 
a
 
n
o
d
e
 
i
t
 
h
a
s
 
f
i
n
i
s
h
e
d
 
w
i
t
h
.
 
 
T
h
o
s
e
 
a
r
e
 
d
i
f
f
e
r
e
n
t
 
o
p
e
r
a
t
i
o
n
s
 
a
n
d
 
t
h
i
s
 
d
o
e
s


 
 
n
o
t
 
s
h
o
w
 
t
h
e
 
s
e
c
o
n
d
 
b
e
h
a
v
e
s
 
t
h
e
 
s
a
m
e
 
w
a
y
.
 
 
W
h
a
t
 
i
t
 
s
e
t
t
l
e
s
 
i
s
 
t
h
a
t
 
t
h
e


 
 
t
r
a
n
s
i
t
i
o
n
 
e
x
i
s
t
s
 
a
n
d
 
w
h
a
t
 
i
t
 
a
c
c
o
u
n
t
s
 
f
o
r
;
 
w
h
a
t
 
i
t
 
l
e
a
v
e
s
 
o
p
e
n
 
i
s
 
w
h
e
t
h
e
r
 
t
h
e


 
 
r
u
n
n
i
n
g
 
f
i
l
e
 
s
y
s
t
e
m
 
e
v
e
r
 
*
c
r
e
a
t
e
s
*
 
t
h
e
 
c
o
n
d
i
t
i
o
n
,
 
g
i
v
e
n
 
t
h
a
t
 
s
t
o
c
k
i
n
g
 
s
t
o
p
s
 
o
n
e


 
 
s
l
o
t
 
s
h
o
r
t
 
o
f
 
t
h
e
 
e
n
d
 
b
y
 
d
e
s
i
g
n
.




 
 
T
h
e
 
r
o
w
 
i
t
 
r
e
p
l
a
c
e
s
 
s
a
i
d
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
"
m
a
y
 
n
o
t
 
e
x
i
s
t
"
.
 
 
I
t
 
d
o
e
s
 
-
-
 
a
n
d
 
t
h
e
 
w
a
y


 
 
i
t
 
w
a
s
 
f
o
u
n
d
 
i
s
 
w
o
r
t
h
 
r
e
c
o
r
d
i
n
g
:
 
t
h
e
 
m
i
s
s
i
n
g
 
i
n
g
r
e
d
i
e
n
t
 
w
a
s
 
n
o
t
 
a
 
m
o
u
n
t
,
 
i
t
 
w
a
s


 
 
a
 
s
t
a
t
e
 
t
h
e
 
*
t
o
o
l
*
 
c
o
u
l
d
 
b
e
 
a
s
k
e
d
 
a
b
o
u
t
.




#
#
#
 
W
h
a
t
 
t
h
e
 
a
u
d
i
t
 
c
h
a
n
g
e
d




*
 
T
h
e
 
i
d
e
n
t
i
t
y
 
a
n
d
 
b
o
t
h
 
o
f
 
i
t
s
 
c
o
m
p
o
n
e
n
t
 
c
l
a
i
m
s
 
m
o
v
e
d
 
f
r
o
m
 
"
m
e
a
s
u
r
e
d
"
 
t
o


 
 
*
*
c
o
n
f
i
r
m
e
d
*
*
,
 
o
n
 
t
h
e
 
e
v
i
d
e
n
c
e
 
o
f
 
`
x
f
s
_
d
b
`
'
s
 
o
w
n
 
b
l
o
c
k
 
w
a
l
k
 
r
a
t
h
e
r
 
t
h
a
n
 
a


 
 
p
a
r
s
e
r
.
 
 
`
b
l
o
c
k
g
e
t
 
-
v
 
-
s
`
 
l
a
b
e
l
s
 
e
v
e
r
y
 
b
l
o
c
k
 
i
n
 
a
 
f
i
l
e
 
s
y
s
t
e
m
 
w
i
t
h
 
w
h
a
t
 
o
w
n
s


 
 
i
t
,
 
a
n
d
 
t
h
r
e
e
 
o
f
 
t
h
o
s
e
 
l
a
b
e
l
s
 
a
r
e
 
t
h
e
 
t
h
r
e
e
 
t
e
r
m
s
,
 
s
o
 
t
h
e
 
w
h
o
l
e
 
i
d
e
n
t
i
t
y
 
i
s
 
n
o
w


 
 
a
r
i
t
h
m
e
t
i
c
 
o
n
 
n
u
m
b
e
r
s
 
`
x
f
s
_
d
b
`
 
p
r
o
d
u
c
e
d
.


*
 
`
R
a
w
D
i
n
o
d
e
:
:
u
n
u
s
e
d
`
 
n
o
 
l
o
n
g
e
r
 
d
e
s
c
r
i
b
e
s
 
i
t
s
e
l
f
 
a
s
 
"
a
 
s
l
o
t
 
t
h
a
t
 
h
a
s
 
n
e
v
e
r
 
b
e
e
n


 
 
u
s
e
d
"
.
 
 
I
t
 
i
s
 
a
 
z
e
r
o
e
d
 
b
u
f
f
e
r
 
f
o
r
 
b
u
i
l
d
i
n
g
 
a
n
 
i
n
o
d
e
 
i
n
;
 
a
 
f
r
e
e
 
i
n
o
d
e
'
s
 
*
s
t
a
t
e
*


 
 
i
s
 
r
e
c
o
r
d
e
d
 
b
y
 
t
h
e
 
t
r
e
e
 
o
f
 
u
s
e
d
 
i
n
o
d
e
 
n
u
m
b
e
r
s
 
a
n
d
 
n
o
t
 
b
y
 
i
t
s
 
b
y
t
e
s
,
 
a
n
d
 
t
h
i
s


 
 
r
e
p
o
s
i
t
o
r
y
'
s
 
i
m
a
g
e
s
 
s
h
o
w
 
i
t
s
 
b
y
t
e
s
 
a
r
e
 
n
o
t
 
z
e
r
o
.


*
 
O
n
e
 
t
r
a
p
 
n
a
m
e
d
:
 
t
h
e
 
t
h
i
r
d
 
t
e
r
m
 
i
s
 
t
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r
'
s
 
*
*
f
r
e
e
 
l
i
s
t
*
*
 
c
o
u
n
t


 
 
(
`
f
l
c
o
u
n
t
`
)
,
 
a
n
d
 
t
h
e
 
f
i
e
l
d
 
b
e
s
i
d
e
 
i
t
 
w
i
t
h
 
a
 
s
i
m
i
l
a
r
 
n
a
m
e
,
 
`
a
g
i
 
f
r
e
e
c
o
u
n
t
`
,
 
i
s


 
 
t
h
e
 
g
r
o
u
p
'
s
 
*
f
r
e
e
 
i
n
o
d
e
s
*
,
 
w
h
i
c
h
 
o
n
 
`
x
f
s
v
4
.
i
m
g
`
 
s
u
m
s
 
t
o
 
2
8
2
4
.
 
 
R
e
a
d
i
n
g
 
t
h
a
t
 
o
n
e


 
 
g
i
v
e
s
 
2
8
2
4
 
w
h
e
r
e
 
t
h
e
 
i
d
e
n
t
i
t
y
 
w
a
n
t
s
 
2
2
,
 
a
n
d
 
i
t
 
l
o
o
k
s
 
e
x
a
c
t
l
y
 
l
i
k
e
 
t
h
e
 
i
d
e
n
t
i
t
y


 
 
b
e
i
n
g
 
w
r
o
n
g
.




-
-
-




#
#
 
H
o
w
 
a
 
f
o
r
m
a
t
 
f
a
c
t
 
i
s
 
e
s
t
a
b
l
i
s
h
e
d
 
h
e
r
e




T
h
e
r
e
 
i
s
 
o
n
e
 
r
u
l
e
 
i
n
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
t
h
a
t
 
e
v
e
r
y
t
h
i
n
g
 
i
n
 
[
M
e
a
s
u
r
e
d


i
n
v
a
r
i
a
n
t
s
]
(
#
m
e
a
s
u
r
e
d
-
i
n
v
a
r
i
a
n
t
s
)
 
a
n
d
 
i
n
 
t
h
e
 
[
a
u
d
i
t
]
(
#
h
o
w
-
s
t
r
o
n
g
-
e
a
c
h
-
m
e
a
s
u
r
e
m
e
n
t
-
i
s
-
a
n
d
-
h
o
w
-
t
o
-
r
e
a
d
-
o
n
e
)


f
o
l
l
o
w
s
,
 
a
n
d
 
i
t
 
i
s
 
w
o
r
t
h
 
s
t
a
t
i
n
g
 
o
n
 
i
t
s
 
o
w
n
 
b
e
c
a
u
s
e
 
i
t
 
w
a
s
 
l
e
a
r
n
e
d
 
t
h
e
 
h
a
r
d
 
w
a
y
 
a
n
d


b
e
c
a
u
s
e
 
i
t
 
i
s
 
r
e
u
s
a
b
l
e
:
 
X
F
S
'
s
 
o
w
n
 
d
i
a
g
n
o
s
t
i
c
 
p
r
o
g
r
a
m
 
i
s
 
t
h
e
 
o
r
a
c
l
e
,
 
a
n
d
 
i
t
 
c
a
n
 
o
n
l
y


b
e
 
a
s
k
e
d
 
a
b
o
u
t
 
s
t
r
u
c
t
u
r
e
s
 
i
t
 
a
l
r
e
a
d
y
 
a
c
c
e
p
t
s
.




>
 
*
*
D
o
 
n
o
t
 
i
n
f
e
r
 
a
n
 
o
n
-
d
i
s
k
 
s
t
r
u
c
t
u
r
e
 
b
y
 
b
u
i
l
d
i
n
g
 
a
 
c
a
n
d
i
d
a
t
e
 
a
n
d
 
r
e
a
d
i
n
g
 
t
h
e


>
 
r
e
j
e
c
t
i
o
n
.
*
*
 
 
G
e
t
 
a
 
s
t
r
u
c
t
u
r
e
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
 
a
l
r
e
a
d
y
 
a
c
c
e
p
t
s
,
 
p
e
r
t
u
r
b
 
o
n
e


>
 
c
o
n
t
r
o
l
l
e
d
 
f
i
e
l
d
,
 
a
n
d
 
r
e
a
d
 
w
h
a
t
 
t
h
e
 
c
h
e
c
k
e
r
 
t
h
e
n
 
s
a
y
s
 
a
b
o
u
t
 
*
t
h
a
t
*
 
s
t
r
u
c
t
u
r
e
.




#
#
#
 
W
h
y
,
 
c
o
n
c
r
e
t
e
l
y




B
u
i
l
d
i
n
g
 
c
a
n
d
i
d
a
t
e
s
 
l
o
o
k
s
 
l
i
k
e
 
t
h
e
 
s
a
m
e
 
m
e
t
h
o
d
 
a
n
d
 
i
s
 
n
o
t
.
 
 
W
h
e
n
 
s
i
x
 
h
a
n
d
-
b
u
i
l
t


r
e
c
o
r
d
s
 
w
e
r
e
 
h
a
n
d
e
d
 
t
o
 
`
x
f
s
_
r
e
p
a
i
r
`
 
f
o
r
 
t
h
e
 
b
-
m
a
p
 
b
l
o
c
k
 
l
a
y
o
u
t
,
 
a
l
l
 
s
i
x
 
w
e
r
e


r
e
f
u
s
e
d
 
—
 
a
n
d
 
*
*
n
o
n
e
 
o
f
 
t
h
e
m
 
w
a
s
 
e
v
e
r
 
r
e
a
d
.
*
*
 
 
T
h
e
 
r
e
f
u
s
a
l
 
d
e
s
c
r
i
b
e
d
 
t
h
e


*
c
o
n
s
t
r
u
c
t
i
o
n
*
,
 
n
o
t
 
t
h
e
 
f
o
r
m
a
t
:
 
t
h
e
 
f
o
r
k
 
p
o
i
n
t
e
d
 
a
t
 
t
h
e
 
n
e
w
 
l
e
a
v
e
s
,
 
b
u
t
 
r
e
p
a
i
r
 
r
e
a
d


v
a
l
u
e
s
 
t
h
a
t
 
w
e
r
e
 
n
o
t
 
t
h
e
 
o
n
e
s
 
w
r
i
t
t
e
n
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
a
 
c
h
e
c
k
e
r
 
d
o
e
s
 
w
h
e
n
 
i
t
 
i
s


l
o
o
k
i
n
g
 
s
o
m
e
w
h
e
r
e
 
e
l
s
e
 
e
n
t
i
r
e
l
y
.
 
 
A
 
t
a
b
l
e
 
o
f
 
s
i
x
 
r
e
j
e
c
t
e
d
 
c
a
n
d
i
d
a
t
e
s
 
r
e
a
d
s
 
e
x
a
c
t
l
y


l
i
k
e
 
s
i
x
 
r
e
j
e
c
t
e
d
 
l
a
y
o
u
t
s
,
 
a
n
d
 
t
h
e
 
t
w
o
 
h
a
v
e
 
n
o
t
h
i
n
g
 
i
n
 
c
o
m
m
o
n
.
 
 
H
a
d
 
i
t
 
b
e
e
n
 
t
a
k
e
n


a
t
 
f
a
c
e
 
v
a
l
u
e
,
 
t
h
e
 
n
e
x
t
 
s
i
x
 
g
u
e
s
s
e
s
 
w
o
u
l
d
 
h
a
v
e
 
b
e
e
n
 
b
u
i
l
t
 
o
n
 
n
o
t
h
i
n
g
.




P
e
r
t
u
r
b
i
n
g
 
t
h
e
 
p
r
i
s
t
i
n
e
,
 
a
c
c
e
p
t
e
d
 
l
e
a
v
e
s
 
i
n
s
t
e
a
d
 
p
r
o
d
u
c
e
d
 
a
n
s
w
e
r
s
 
o
n
 
t
h
e
 
f
i
r
s
t


t
r
y
,
 
b
e
c
a
u
s
e
 
t
h
e
 
o
n
l
y
 
t
h
i
n
g
 
d
i
f
f
e
r
i
n
g
 
f
r
o
m
 
a
 
s
t
r
u
c
t
u
r
e
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
 
i
t
s
e
l
f
 
w
r
o
t
e


i
s
 
t
h
e
 
f
i
e
l
d
 
u
n
d
e
r
 
t
e
s
t
.
 
 
T
h
a
t
 
i
s
 
w
h
a
t
 
m
a
k
e
s
 
t
h
e
 
e
v
i
d
e
n
c
e
 
a
t
t
r
i
b
u
t
a
b
l
e
.




#
#
#
 
H
o
w
 
t
o
 
r
u
n
 
o
n
e




1
.
 
*
*
F
i
n
d
 
a
c
c
e
p
t
e
d
 
m
e
t
a
d
a
t
a
 
t
h
a
t
 
u
s
e
s
 
t
h
e
 
s
t
r
u
c
t
u
r
e
.
*
*
 
 
N
o
t
 
a
 
f
i
x
t
u
r
e
 
a
n
d
 
n
o
t
 
a


 
 
 
c
o
n
s
t
r
u
c
t
i
o
n
 
—
 
r
e
a
l
 
b
y
t
e
s
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
a
l
r
e
a
d
y
 
a
c
c
e
p
t
s
.
 
 
`
x
f
s
_
d
b
 
-
c


 
 
 
'
b
l
o
c
k
g
e
t
 
-
v
 
-
s
'
`
 
f
i
n
d
s
 
t
h
e
 
i
n
o
d
e
s
 
t
h
a
t
 
u
s
e
 
a
 
g
i
v
e
n
 
f
o
r
m
a
t
;
 
h
e
r
e
 
i
t
 
f
o
u
n
d
 
t
h
e


 
 
 
t
h
r
e
e
 
f
i
l
e
s
 
i
n
 
`
x
f
s
v
4
.
i
m
g
`
 
w
h
o
s
e
 
d
a
t
a
 
f
o
r
k
 
i
s
 
a
 
b
-
t
r
e
e
.


2
.
 
*
*
V
e
r
i
f
y
 
t
h
e
 
r
e
w
r
i
t
e
 
l
a
n
d
e
d
,
 
b
e
f
o
r
e
 
r
e
a
d
i
n
g
 
a
n
y
 
v
e
r
d
i
c
t
.
*
*
 
 
A
s
k
 
`
x
f
s
_
d
b
`
 
w
h
a
t
 
i
t


 
 
 
n
o
w
 
s
e
e
s
.
 
 
A
 
c
o
m
p
l
a
i
n
t
 
a
b
o
u
t
 
a
 
s
t
r
u
c
t
u
r
e
 
y
o
u
 
d
i
d
 
n
o
t
 
m
a
n
a
g
e
 
t
o
 
w
r
i
t
e
 
i
s
 
a


 
 
 
s
t
a
t
e
m
e
n
t
 
a
b
o
u
t
 
y
o
u
,
 
n
o
t
 
a
b
o
u
t
 
X
F
S
.


3
.
 
*
*
C
h
a
n
g
e
 
o
n
e
 
f
i
e
l
d
,
 
t
o
 
a
 
v
a
l
u
e
 
t
h
a
t
 
i
s
 
u
n
m
i
s
t
a
k
a
b
l
y
 
w
r
o
n
g
.
*
*
 
 
`
0
x
1
1
2
2
…
`
 
r
a
t
h
e
r


 
 
 
t
h
a
n
 
a
 
p
l
a
u
s
i
b
l
e
 
b
l
o
c
k
 
n
u
m
b
e
r
,
 
s
o
 
t
h
e
 
m
e
s
s
a
g
e
 
h
a
s
 
t
o
 
n
a
m
e
 
i
t
.


4
.
 
*
*
R
e
a
d
 
t
h
e
 
m
e
s
s
a
g
e
 
f
o
r
 
t
h
e
 
f
i
e
l
d
 
y
o
u
 
c
h
a
n
g
e
d
,
 
a
n
d
 
o
n
l
y
 
t
h
a
t
 
f
i
e
l
d
.
*
*
 
 
W
h
e
r
e
 
t
h
e


 
 
 
c
h
e
c
k
e
r
 
r
e
p
o
r
t
s
 
a
n
 
*
o
f
f
s
e
t
*
 
o
r
 
a
 
*
d
e
r
i
v
e
d
*
 
n
u
m
b
e
r
,
 
c
o
n
v
e
r
t
 
i
t
 
b
a
c
k
 
b
e
f
o
r
e


 
 
 
c
o
m
p
a
r
i
n
g
.


5
.
 
*
*
T
r
e
a
t
 
t
h
e
 
c
h
e
c
k
e
r
'
s
 
n
u
m
b
e
r
s
 
a
s
 
d
e
c
o
d
e
d
 
v
a
l
u
e
s
,
 
n
e
v
e
r
 
a
s
 
f
i
e
l
d
s
.
*
*
 
 
T
h
i
s
 
i
s


 
 
 
t
h
e
 
t
r
a
p
.
 
 
R
e
p
a
i
r
 
r
e
p
o
r
t
e
d
 
a
 
"
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
"
 
o
f
 
`
0
x
1
8
8
0
0
0
0
0
0
c
4
8
8
`
 
f
o
r
 
a


 
 
 
b
l
o
c
k
 
w
h
o
s
e
 
o
n
-
d
i
s
k
 
b
y
t
e
s
 
a
r
e
 
`
0
0
0
0
0
0
0
0
0
0
0
0
c
4
8
b
`
:
 
t
h
e
 
n
u
m
b
e
r
 
s
h
a
r
e
s
 
d
i
g
i
t
s
 
w
i
t
h


 
 
 
t
w
o
 
d
i
f
f
e
r
e
n
t
 
r
e
g
i
o
n
s
 
a
n
d
 
i
s
 
n
e
i
t
h
e
r
,
 
s
o
 
i
t
 
i
s
 
*
a
s
s
e
m
b
l
e
d
*
.
 
 
R
e
a
d
i
n
g
 
i
t
 
a
s
 
a


 
 
 
f
i
e
l
d
 
i
s
 
t
h
e
 
s
a
m
e
 
f
a
l
s
e
 
i
n
f
e
r
e
n
c
e
 
t
h
e
 
w
h
o
l
e
 
e
x
e
r
c
i
s
e
 
e
x
i
s
t
s
 
t
o
 
a
v
o
i
d
,
 
o
n
e
 
s
t
e
p


 
 
 
r
e
m
o
v
e
d
 
f
r
o
m
 
w
h
e
r
e
 
i
t
 
f
i
r
s
t
 
a
p
p
e
a
r
e
d
.


6
.
 
*
*
W
r
i
t
e
 
d
o
w
n
 
w
h
a
t
 
t
h
e
 
e
x
p
e
r
i
m
e
n
t
 
c
a
n
n
o
t
 
s
h
o
w
.
*
*
 
 
A
 
r
e
f
u
s
e
d
 
c
a
n
d
i
d
a
t
e
 
w
h
o
s
e


 
 
 
v
e
r
d
i
c
t
 
i
s
 
n
o
t
 
a
b
o
u
t
 
t
h
e
 
c
a
n
d
i
d
a
t
e
 
i
s
 
n
o
t
 
e
v
i
d
e
n
c
e
,
 
a
n
d
 
s
a
y
i
n
g
 
s
o
 
i
s
 
c
h
e
a
p
e
r
 
t
h
a
n


 
 
 
l
e
t
t
i
n
g
 
t
h
e
 
n
e
x
t
 
p
e
r
s
o
n
 
c
o
u
n
t
 
i
t
.




#
#
#
 
W
h
e
r
e
 
e
l
s
e
 
t
h
i
s
 
a
p
p
l
i
e
s




S
e
v
e
r
a
l
 
t
h
i
n
g
s
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
r
e
c
o
r
d
s
 
w
e
r
e
 
e
s
t
a
b
l
i
s
h
e
d
 
e
x
a
c
t
l
y
 
t
h
i
s
 
w
a
y
,
 
a
n
d
 
e
a
c
h


w
a
s
 
a
 
c
a
s
e
 
w
h
e
r
e
 
t
h
e
 
o
b
v
i
o
u
s
 
r
e
a
d
i
n
g
 
w
o
u
l
d
 
h
a
v
e
 
b
e
e
n
 
w
r
o
n
g
:




|
 
f
a
c
t
 
|
 
h
o
w
 
i
t
 
w
a
s
 
p
i
n
n
e
d
 
|


|
:
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
t
h
e
 
f
r
e
e
 
l
i
s
t
'
s
 
l
i
v
e
 
w
i
n
d
o
w
 
i
s
 
a
 
s
l
i
c
e
 
s
t
a
r
t
i
n
g
 
a
t
 
8
5
 
|
 
`
x
f
s
_
d
b
`
 
p
r
i
n
t
s
 
`
f
l
f
i
r
s
t
 
=
 
8
5
`
 
f
o
r
 
a
 
g
r
o
u
p
 
t
h
e
 
h
e
a
d
e
r
 
s
a
y
s
 
s
o
 
|


|
 
a
 
f
r
e
e
 
i
n
o
d
e
'
s
 
`
n
e
x
t
_
u
n
l
i
n
k
e
d
`
 
i
s
 
a
t
 
o
f
f
s
e
t
 
9
6
 
|
 
p
e
r
t
u
r
b
 
e
a
c
h
 
o
f
f
s
e
t
 
o
f
 
a
n
 
a
c
c
e
p
t
e
d
 
s
l
o
t
;
 
r
e
p
a
i
r
 
n
a
m
e
s
 
t
h
e
 
o
n
e
 
i
t
 
r
e
a
d
 
|


|
 
v
e
r
s
i
o
n
 
3
 
i
s
 
w
r
o
n
g
 
f
o
r
 
a
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
 
|
 
w
r
i
t
e
 
3
 
i
n
t
o
 
a
n
 
a
c
c
e
p
t
e
d
 
s
l
o
t
 
a
n
d
 
a
s
k
 
|


|
 
a
 
n
o
n
-
r
o
o
t
 
l
e
a
f
 
n
e
e
d
s
 
3
1
 
r
e
c
o
r
d
s
,
 
a
 
r
o
o
t
 
l
e
a
f
 
n
o
n
e
 
|
 
s
h
r
i
n
k
 
a
n
 
a
c
c
e
p
t
e
d
 
l
e
a
f
,
 
b
o
t
h
 
c
a
s
e
s
,
 
a
n
d
 
r
e
a
d
 
r
e
p
a
i
r
'
s
 
`
m
i
n
=
`
/
`
m
a
x
=
`
 
|


|
 
a
 
f
u
l
l
 
f
r
e
e
 
l
i
s
t
'
s
 
b
l
o
c
k
s
 
b
e
c
o
m
e
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
 
|
 
f
i
l
l
 
a
n
 
a
c
c
e
p
t
e
d
 
l
i
s
t
 
t
o
 
t
h
e
 
e
n
d
 
o
f
 
t
h
e
 
a
r
r
a
y
 
a
n
d
 
l
e
t
 
t
h
e
 
t
o
o
l
 
r
e
b
u
i
l
d
 
i
t
 
|


|
 
t
h
e
 
d
e
v
i
c
e
-
w
i
d
e
 
f
r
e
e
 
c
o
u
n
t
 
h
a
s
 
t
h
r
e
e
 
t
e
r
m
s
 
|
 
`
x
f
s
_
d
b
`
'
s
 
o
w
n
 
b
l
o
c
k
 
w
a
l
k
,
 
n
o
t
 
t
h
i
s
 
c
o
d
e
'
s
 
r
e
a
d
e
r
 
a
n
d
 
n
o
t
 
a
 
p
a
r
s
e
r
 
|




T
h
e
 
c
o
m
m
o
n
 
s
h
a
p
e
 
i
s
 
t
h
a
t
 
t
h
e
 
f
i
r
s
t
 
r
e
a
d
i
n
g
 
w
a
s
 
w
r
o
n
g
,
 
a
n
d
 
i
n
 
e
a
c
h
 
c
a
s
e
 
i
t
 
w
a
s
 
w
r
o
n
g


i
n
 
a
 
w
a
y
 
t
h
a
t
 
l
o
o
k
e
d
 
r
i
g
h
t
.




#
#
 
M
e
a
s
u
r
e
d
 
i
n
v
a
r
i
a
n
t
s




E
v
e
r
y
t
h
i
n
g
 
i
n
 
t
h
i
s
 
s
e
c
t
i
o
n
 
w
a
s
 
e
s
t
a
b
l
i
s
h
e
d
 
b
y
 
r
e
a
d
i
n
g
 
i
m
a
g
e
s
 
p
r
o
d
u
c
e
d
 
b
y


`
m
k
f
s
.
x
f
s
`
 
o
r
 
s
h
i
p
p
e
d
 
i
n
 
`
r
e
s
o
u
r
c
e
s
/
`
,
 
a
n
d
 
b
y
 
l
e
t
t
i
n
g
 
`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
b
u
i
l
d
 
t
h
e
m
.


T
h
e
 
n
u
m
b
e
r
s
 
a
r
e
 
q
u
o
t
e
d
 
f
r
o
m
 
a
 
n
a
t
i
v
e
 
t
o
o
l
 
w
h
e
r
e
v
e
r
 
o
n
e
 
c
a
n
 
b
e
 
a
s
k
e
d
 
-
-
 
s
e
e


[
H
o
w
 
s
t
r
o
n
g
 
e
a
c
h
 
m
e
a
s
u
r
e
m
e
n
t
 
i
s
]
(
#
h
o
w
-
s
t
r
o
n
g
-
e
a
c
h
-
m
e
a
s
u
r
e
m
e
n
t
-
i
s
-
a
n
d
-
h
o
w
-
t
o
-
r
e
a
d
-
o
n
e
)


f
o
r
 
w
h
a
t
 
s
t
a
n
d
s
 
b
e
h
i
n
d
 
e
a
c
h
 
o
n
e
 
a
n
d
 
w
h
a
t
 
d
o
e
s
 
n
o
t
.




#
#
#
 
`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
i
s
 
e
x
a
c
t
l
y
 
t
h
e
 
s
u
m
 
o
f
 
t
h
e
 
b
n
o
 
t
r
e
e
'
s
 
r
e
c
o
r
d
s




M
e
a
s
u
r
e
d
 
i
n
 
*
*
e
v
e
r
y
*
*
 
g
r
o
u
p
 
o
f
 
a
l
l
 
f
o
u
r
 
i
m
a
g
e
s
,
 
i
n
c
l
u
d
i
n
g
 
t
h
e
 
o
n
e
 
`
x
f
s
_
r
e
p
a
i
r
`


r
e
b
u
i
l
t
:




|
 
I
m
a
g
e
 
|
 
G
r
o
u
p
s
 
|
 
`
s
u
m
(
A
G
F
 
f
r
e
e
b
l
k
s
)
`
 
|
 
`
s
u
m
(
b
n
o
 
r
e
c
o
r
d
 
l
e
n
g
t
h
s
)
`
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
4
 
|
 
9
0
2
7
7
 
|
 
9
0
2
7
7
 
|


|
 
`
x
f
s
_
w
r
i
t
a
b
l
e
.
i
m
g
`
 
|
 
4
 
|
 
4
8
3
1
2
2
 
|
 
4
8
3
1
2
2
 
|


|
 
`
x
f
s
_
4
k
n
.
i
m
g
`
 
|
 
4
 
|
 
1
4
9
6
2
 
|
 
1
4
9
6
2
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
f
t
e
r
 
`
x
f
s
_
r
e
p
a
i
r
`
 
|
 
4
 
|
 
9
0
1
6
4
 
|
 
9
0
1
6
4
 
|




P
e
r
 
g
r
o
u
p
 
t
h
e
 
a
g
r
e
e
m
e
n
t
 
i
s
 
e
x
a
c
t
,
 
n
o
t
 
m
e
r
e
l
y
 
i
n
 
t
o
t
a
l
.




*
*
C
o
n
s
e
q
u
e
n
c
e
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
 
o
p
p
o
s
i
t
e
 
o
f
 
w
h
a
t
 
a
n
 
e
a
r
l
i
e
r
 
v
e
r
s
i
o
n
 
o
f
 
t
h
i
s


d
o
c
u
m
e
n
t
 
s
a
i
d
:
*
*
 
t
h
e
 
b
l
o
c
k
s
 
s
i
t
t
i
n
g
 
o
n
 
a
n
 
A
G
F
L
 
a
r
e
 
*
n
o
t
*
 
i
n
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e


t
r
e
e
s
.
 
 
I
n
 
g
r
o
u
p
 
0
 
o
f
 
`
x
f
s
v
4
.
i
m
g
`
 
t
h
e
 
b
n
o
 
l
e
a
f
 
h
o
l
d
s
 
1
9
 
r
e
c
o
r
d
s
 
t
o
t
a
l
l
i
n
g


3
0
1
4
4
 
b
l
o
c
k
s
 
a
n
d
 
t
h
e
 
h
e
a
d
e
r
 
s
a
y
s
 
`
f
r
e
e
b
l
k
s
 
=
 
3
0
1
4
4
`
,
 
w
h
i
l
e
 
t
h
e
 
f
r
e
e
 
l
i
s
t
 
h
o
l
d
s


b
l
o
c
k
s
 
7
,
 
8
,
 
9
 
a
n
d
 
1
0
 
—
 
w
h
i
c
h
 
a
p
p
e
a
r
 
i
n
 
n
o
 
b
n
o
 
r
e
c
o
r
d
.
 
 
A
 
b
l
o
c
k
 
o
n
 
t
h
e
 
l
i
s
t
 
i
s


t
h
e
r
e
f
o
r
e
 
n
e
i
t
h
e
r
 
a
 
f
r
e
e
 
e
x
t
e
n
t
 
n
o
r
 
a
 
l
i
v
e
 
n
o
d
e
,
 
a
n
d
 
t
h
e
 
g
r
o
u
p
'
s
 
c
o
u
n
t
 
d
o
e
s
 
n
o
t


i
n
c
l
u
d
e
 
i
t
.




#
#
#
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
i
s
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
'
 
b
l
o
c
k
s
,
 
l
e
s
s
 
t
h
e
i
r
 
t
w
o
 
r
o
o
t
s




M
e
a
s
u
r
e
d
 
i
n
 
e
v
e
r
y
 
g
r
o
u
p
 
o
f
 
a
l
l
 
f
o
u
r
 
i
m
a
g
e
s
:




|
 
I
m
a
g
e
 
|
 
G
r
o
u
p
 
|
 
`
b
n
o
 
b
l
o
c
k
s
`
 
|
 
`
c
n
t
 
b
l
o
c
k
s
`
 
|
 
t
o
t
a
l
 
−
 
2
 
|
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
0
 
|
 
1
 
|
 
1
 
|
 
0
 
|
 
0
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
1
 
|
 
3
0
 
|
 
3
0
 
|
 
5
8
 
|
 
5
8
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
2
 
|
 
1
 
|
 
1
 
|
 
0
 
|
 
0
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
3
 
|
 
1
3
4
 
|
 
1
3
5
 
|
 
2
6
7
 
|
 
2
6
7
 
|


|
 
a
f
t
e
r
 
`
x
f
s
_
r
e
p
a
i
r
`
 
|
 
3
 
|
 
1
7
6
 
|
 
1
7
6
 
|
 
3
5
0
 
|
 
3
5
0
 
|




S
o
 
t
h
e
 
f
i
e
l
d
 
c
o
u
n
t
s
 
t
h
e
 
b
l
o
c
k
s
 
b
o
t
h
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
o
c
c
u
p
y
 
*
*
o
t
h
e
r
 
t
h
a
n
 
t
h
e
i
r


r
o
o
t
 
b
l
o
c
k
s
*
*
.
 
 
T
h
i
s
 
i
s
 
a
 
m
e
a
s
u
r
e
m
e
n
t
,
 
n
o
t
 
a
 
f
o
r
m
u
l
a
 
c
h
o
s
e
n
 
t
o
 
f
i
t
:
 
i
t
 
w
a
s


c
h
e
c
k
e
d
 
p
e
r
 
g
r
o
u
p
 
a
g
a
i
n
s
t
 
t
h
e
 
t
r
e
e
 
t
h
e
 
i
m
a
g
e
 
a
c
t
u
a
l
l
y
 
h
o
l
d
s
,
 
o
n
 
a
 
5
1
2
-
b
y
t
e


b
l
o
c
k
 
i
m
a
g
e
,
 
a
 
4
0
9
6
-
b
y
t
e
 
b
l
o
c
k
 
i
m
a
g
e
,
 
a
n
d
 
a
n
 
i
m
a
g
e
 
X
F
S
 
i
t
s
e
l
f
 
r
e
w
r
o
t
e
.




#
#
#
 
T
h
e
 
s
u
p
e
r
b
l
o
c
k
'
s
 
f
r
e
e
 
c
o
u
n
t
 
h
a
s
 
t
h
r
e
e
 
t
e
r
m
s
,
 
a
n
d
 
a
l
l
 
t
h
r
e
e
 
a
r
e
 
m
e
a
s
u
r
e
d




`
s
b
_
f
d
b
l
o
c
k
s
`
 
i
s
 
*
*
n
o
t
*
*
 
t
h
e
 
s
u
m
 
o
f
 
t
h
e
 
g
r
o
u
p
s
'
 
`
f
r
e
e
b
l
k
s
`
 
—
 
e
a
r
l
i
e
r
 
v
e
r
s
i
o
n
s
 
o
f


t
h
i
s
 
d
o
c
u
m
e
n
t
 
r
e
c
o
r
d
e
d
 
t
h
e
 
g
a
p
 
a
n
d
 
c
o
u
l
d
 
n
o
t
 
a
c
c
o
u
n
t
 
f
o
r
 
i
t
.
 
 
I
t
 
i
s
:




`
`
`
t
e
x
t


s
b
_
f
d
b
l
o
c
k
s
 
=
=
 
s
u
m
 
o
v
e
r
 
g
r
o
u
p
s
 
o
f
 
(
 
a
g
f
_
f
r
e
e
b
l
k
s


 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
+
 
a
g
f
_
b
t
r
e
e
b
l
k
s


 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
+
 
a
g
f
_
f
l
c
o
u
n
t
 
)


`
`
`




E
x
a
c
t
l
y
,
 
o
n
 
e
v
e
r
y
 
i
m
a
g
e
 
m
e
a
s
u
r
e
d
:




|
 
I
m
a
g
e
 
|
 
`
s
u
m
(
f
r
e
e
b
l
k
s
)
`
 
|
 
`
s
u
m
(
b
t
r
e
e
b
l
k
s
)
`
 
|
 
`
s
u
m
(
f
l
c
o
u
n
t
)
`
 
|
 
t
o
t
a
l
 
|
 
`
s
b
_
f
d
b
l
o
c
k
s
`
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
9
0
2
7
7
 
|
 
3
2
5
 
|
 
2
2
 
|
 
9
0
6
2
4
 
|
 
9
0
6
2
4
 
|


|
 
`
x
f
s
_
w
r
i
t
a
b
l
e
.
i
m
g
`
 
|
 
4
8
3
1
2
2
 
|
 
0
 
|
 
1
6
 
|
 
4
8
3
1
3
8
 
|
 
4
8
3
1
3
8
 
|


|
 
`
x
f
s
_
4
k
n
.
i
m
g
`
 
|
 
1
4
9
6
2
 
|
 
0
 
|
 
1
6
 
|
 
1
4
9
7
8
 
|
 
1
4
9
7
8
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
f
t
e
r
 
`
x
f
s
_
r
e
p
a
i
r
`
 
|
 
9
0
1
6
4
 
|
 
4
2
2
 
|
 
3
6
 
|
 
9
0
6
2
2
 
|
 
9
0
6
2
2
 
|




T
h
e
 
t
h
r
e
e
 
t
e
r
m
s
 
a
r
e
 
t
h
e
 
t
h
r
e
e
 
p
l
a
c
e
s
 
a
 
f
r
e
e
 
b
l
o
c
k
 
c
a
n
 
b
e
 
a
c
c
o
u
n
t
e
d
 
f
o
r
:
 
f
r
e
e


s
p
a
c
e
 
i
n
 
a
 
b
n
o
 
t
r
e
e
,
 
o
w
n
e
d
 
b
y
 
o
n
e
 
o
f
 
t
h
e
 
t
w
o
 
f
r
e
e
 
s
p
a
c
e
 
b
t
r
e
e
s
,
 
o
r
 
r
e
s
e
r
v
e
d
 
o
n


a
n
 
A
G
F
L
.
 
 
A
 
b
l
o
c
k
 
t
h
a
t
 
i
s
 
n
o
n
e
 
o
f
 
t
h
o
s
e
 
t
h
r
e
e
 
i
s
 
a
 
b
l
o
c
k
 
s
o
m
e
t
h
i
n
g
 
e
l
s
e
 
o
w
n
s
.




T
h
i
s
 
i
d
e
n
t
i
t
y
 
i
s
 
t
h
e
 
r
e
a
s
o
n
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
'
s
 
o
w
n
e
r
s
h
i
p
 
c
a
n
 
b
e
 
r
e
a
s
o
n
e
d
 
a
b
o
u
t


a
t
 
a
l
l
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
 
a
r
i
t
h
m
e
t
i
c
 
b
e
h
i
n
d
 
[
B
l
o
c
k
e
d
]
(
#
b
l
o
c
k
e
d
)
.
 
 
I
t
 
i
s


a
 
l
o
c
a
l
 
d
e
v
i
c
e
-
w
i
d
e
 
i
d
e
n
t
i
t
y
,
 
*
*
n
o
t
*
*
 
a
 
c
l
a
i
m
 
t
h
a
t


`
s
b
_
f
d
b
l
o
c
k
s
 
=
=
 
s
u
m
(
a
g
f
_
f
r
e
e
b
l
k
s
)
`
,
 
w
h
i
c
h
 
i
s
 
f
a
l
s
e
 
o
n
 
a
l
l
 
f
o
u
r
 
i
m
a
g
e
s
.




#
#
#
 
T
h
e
 
f
r
e
e
 
l
i
s
t
'
s
 
l
i
v
e
 
w
i
n
d
o
w
 
i
s
 
a
 
q
u
e
u
e
 
o
v
e
r
 
t
h
e
 
a
r
r
a
y
,
 
a
n
d
 
i
t
 
d
o
e
s
 
n
o
t
 
s
t
a
r
t
 
a
t
 
z
e
r
o




`
f
l
f
i
r
s
t
 
.
.
=
 
f
l
l
a
s
t
`
 
i
s
 
t
h
e
 
l
i
v
e
 
p
o
r
t
i
o
n
 
o
f
 
t
h
e
 
a
r
r
a
y
 
a
n
d
 
`
f
l
c
o
u
n
t
`
 
i
s
 
h
o
w
 
m
a
n
y


o
f
 
t
h
o
s
e
 
s
l
o
t
s
 
h
o
l
d
 
s
o
m
e
t
h
i
n
g
.
 
 
T
h
a
t
 
i
s
 
t
h
e
 
w
h
o
l
e
 
m
o
d
e
l
,
 
a
n
d
 
i
t
 
i
s
 
w
h
a
t


[
`
A
g
f
l
W
i
n
d
o
w
`
]
(
.
.
/
.
.
/
s
r
c
/
l
i
b
x
f
u
s
e
/
a
l
l
o
c
/
a
g
f
l
.
r
s
)
 
i
s
.
 
 
W
h
a
t
 
t
h
e
 
m
e
a
s
u
r
e
m
e
n
t
s
 
a
d
d


i
s
 
t
h
a
t
 
t
h
e
 
w
i
n
d
o
w
 
i
s
 
g
e
n
u
i
n
e
l
y
 
a
 
*
s
l
i
c
e
*
,
 
n
o
t
 
t
h
e
 
w
h
o
l
e
 
a
r
r
a
y
,
 
a
n
d
 
t
h
a
t
 
i
t
s


c
o
n
t
e
n
t
s
 
a
r
e
 
i
n
 
n
o
 
p
a
r
t
i
c
u
l
a
r
 
o
r
d
e
r
:




|
 
I
m
a
g
e
 
/
 
g
r
o
u
p
 
|
 
w
i
n
d
o
w
 
|
 
l
i
v
e
 
e
n
t
r
i
e
s
 
|


|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
g
 
0
 
|
 
`
(
1
,
 
4
,
 
4
)
`
 
|
 
`
7
,
 
8
,
 
9
,
 
1
0
`
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
g
 
1
 
|
 
`
(
8
5
,
 
9
0
,
 
6
)
`
 
|
 
`
1
1
9
9
8
,
 
1
1
9
9
9
,
 
5
7
4
,
 
5
7
3
,
 
1
4
8
2
,
 
1
4
8
1
`
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
g
 
2
 
|
 
`
(
1
,
 
4
,
 
4
)
`
 
|
 
`
4
8
1
3
,
 
4
8
1
4
,
 
4
8
1
5
,
 
4
8
1
6
`
 
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
a
g
 
3
 
|
 
`
(
2
6
,
 
3
3
,
 
8
)
`
 
|
 
`
9
4
5
,
 
9
4
7
,
 
9
4
9
,
 
9
5
1
,
 
9
5
3
,
 
3
8
8
,
 
9
5
5
,
 
9
4
1
`
 
|


|
 
`
x
f
s
_
4
k
n
.
i
m
g
`
 
a
g
 
0
 
|
 
`
(
1
,
 
4
,
 
4
)
`
 
|
 
`
9
,
 
1
0
,
 
1
1
,
 
1
2
`
 
|




S
o
 
`
f
l
f
i
r
s
t
`
 
i
s
 
8
5
 
a
n
d
 
2
6
 
i
n
 
t
w
o
 
o
f
 
t
h
e
s
e
 
g
r
o
u
p
s
,
 
a
n
d
 
t
h
e
 
e
n
t
r
i
e
s
 
a
r
e
 
n
e
i
t
h
e
r


s
o
r
t
e
d
 
n
o
r
 
e
v
e
n
 
g
r
o
u
p
e
d
.
 
 
A
n
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
 
t
h
a
t
 
a
s
s
u
m
e
s
 
`
f
i
r
s
t
 
=
=
 
0
`
,
 
o
r
 
t
h
a
t


s
c
a
n
s
 
t
h
e
 
a
r
r
a
y
 
f
o
r
 
n
o
n
-
n
u
l
l
 
s
l
o
t
s
,
 
i
s
 
w
r
o
n
g
 
o
n
 
r
e
a
l
 
i
m
a
g
e
s
 
a
n
d
 
w
r
o
n
g
 
i
n
 
a
 
w
a
y


t
h
a
t
 
h
a
n
d
s
 
o
u
t
 
b
l
o
c
k
s
 
n
o
b
o
d
y
 
r
e
s
e
r
v
e
d
.




#
#
#
 
W
h
e
t
h
e
r
 
t
h
e
 
l
i
s
t
 
c
a
r
r
i
e
s
 
a
 
h
e
a
d
e
r
 
i
s
 
a
 
p
r
o
p
e
r
t
y
 
o
f
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
,
 
a
n
d
 
b
o
t
h
 
e
x
i
s
t




|
 
I
m
a
g
e
 
|
 
V
e
r
s
i
o
n
 
|
 
F
r
e
e
 
l
i
s
t
 
b
l
o
c
k
 
a
t
 
s
e
c
t
o
r
 
3
 
o
f
 
g
r
o
u
p
 
0
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
`
x
f
s
v
4
.
i
m
g
`
 
|
 
4
 
|
 
`
f
f
f
f
f
f
f
f
 
0
0
0
0
0
0
0
7
 
0
0
0
0
0
0
0
8
 
0
0
0
0
0
0
0
9
 
0
0
0
0
0
0
0
a
 
f
f
f
f
…
`
 
—
 
a
 
b
a
r
e
 
a
r
r
a
y
,
 
*
*
n
o
*
*
 
`
X
A
F
L
`
 
m
a
g
i
c
 
|


|
 
`
x
f
s
_
4
k
n
.
i
m
g
`
 
|
 
5
 
|
 
`
5
8
4
1
4
6
4
c
 
…
`
 
(
`
X
A
F
L
`
,
 
s
e
q
u
e
n
c
e
,
 
u
u
i
d
,
 
l
s
n
,
 
c
r
c
)
 
t
h
e
n
 
t
h
e
 
a
r
r
a
y
 
a
t
 
o
f
f
s
e
t
 
3
6
 
|




`
A
g
f
l
:
:
f
r
o
m
_
b
y
t
e
s
`
 
d
e
c
i
d
e
s
 
f
r
o
m
 
t
h
e
 
b
l
o
c
k
 
r
a
t
h
e
r
 
t
h
a
n
 
a
s
s
u
m
i
n
g
,
 
w
h
i
c
h
 
i
s
 
c
o
r
r
e
c
t
.


W
h
a
t
 
i
s
 
*
n
o
t
*
 
c
o
r
r
e
c
t
 
i
s
 
t
h
e
 
c
l
a
i
m
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
u
s
e
d
 
t
o
 
m
a
k
e
,
 
r
e
p
e
a
t
e
d
 
i
n


s
e
v
e
r
a
l
 
p
l
a
c
e
s
,
 
t
h
a
t
 
*
*
n
o
 
i
m
a
g
e
 
i
n
 
t
h
e
 
r
e
p
o
s
i
t
o
r
y
 
h
a
s
 
a
 
w
r
i
t
t
e
n
 
f
r
e
e
 
l
i
s
t
*
*
:


e
v
e
r
y
 
i
m
a
g
e
 
m
e
a
s
u
r
e
d
 
h
a
s
 
a
 
w
r
i
t
t
e
n
 
a
r
r
a
y
 
h
o
l
d
i
n
g
 
e
x
a
c
t
l
y
 
t
h
e
 
b
l
o
c
k
s
 
i
t
s
 
w
i
n
d
o
w


n
a
m
e
s
.
 
 
T
h
e
y
 
l
a
c
k
 
a
 
*
m
a
g
i
c
 
n
u
m
b
e
r
*
,
 
a
n
d
 
c
o
n
f
u
s
i
n
g
 
"
h
a
s
 
n
o
 
m
a
g
i
c
"
 
w
i
t
h
 
"
h
a
s


n
e
v
e
r
 
b
e
e
n
 
w
r
i
t
t
e
n
"
 
i
s
 
w
h
a
t
 
l
e
d
 
t
o
 
t
h
e
 
"
t
h
e
 
A
G
F
L
 
i
s
 
n
o
t
 
i
n
i
t
i
a
l
i
s
e
d
"
 
r
e
a
d
i
n
g
.


T
h
e
 
r
i
g
h
t
 
t
e
s
t
 
f
o
r
 
"
i
s
 
t
h
i
s
 
s
l
o
t
 
u
s
a
b
l
e
"
 
i
s
 
w
h
e
t
h
e
r
 
t
h
e
 
s
l
o
t
 
h
o
l
d
s
 
a
 
b
l
o
c
k


n
u
m
b
e
r
 
t
h
a
t
 
i
s
 
n
e
i
t
h
e
r
 
t
h
e
 
n
u
l
l
 
b
l
o
c
k
 
n
o
r
 
z
e
r
o
 
—
 
n
o
t
 
w
h
e
t
h
e
r
 
t
h
e
 
b
l
o
c
k
 
o
p
e
n
s


w
i
t
h
 
a
 
m
a
g
i
c
 
n
u
m
b
e
r
.




Z
e
r
o
 
i
s
 
w
o
r
t
h
 
a
 
s
e
p
a
r
a
t
e
 
w
o
r
d
,
 
b
e
c
a
u
s
e
 
i
t
 
i
s
 
t
h
e
 
d
a
n
g
e
r
o
u
s
 
c
a
s
e
.
 
 
A
 
f
r
e
e
 
l
i
s
t


b
l
o
c
k
 
t
h
a
t
 
h
a
s
 
n
e
v
e
r
 
b
e
e
n
 
w
r
i
t
t
e
n
 
i
s
 
a
 
r
u
n
 
o
f
 
z
e
r
o
e
s
,
 
a
n
d
 
a
 
*
*
z
e
r
o
 
s
l
o
t
 
i
s


b
l
o
c
k
 
0
*
*
 
—
 
t
h
e
 
b
l
o
c
k
 
h
o
l
d
i
n
g
 
t
h
e
 
g
r
o
u
p
'
s
 
o
w
n
 
h
e
a
d
e
r
s
,
 
a
n
d
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
 
i
n


g
r
o
u
p
 
0
.
 
 
B
e
l
i
e
v
i
n
g
 
s
u
c
h
 
a
 
s
l
o
t
 
h
a
n
d
s
 
o
u
t
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
.
 
 
E
v
e
r
y
 
p
a
t
h
 
t
h
a
t


t
a
k
e
s
 
a
 
b
l
o
c
k
 
f
r
o
m
 
t
h
e
 
l
i
s
t
 
r
e
f
u
s
e
s
 
b
o
t
h
 
t
h
e
 
n
u
l
l
 
b
l
o
c
k
 
a
n
d
 
z
e
r
o
.




#
#
#
 
X
F
S
'
s
 
o
w
n
 
g
r
o
u
p
 
r
e
b
u
i
l
d
 
m
o
v
e
s
 
t
h
e
 
c
o
u
n
t
e
r
s
 
i
n
 
t
h
e
 
e
x
p
e
c
t
e
d
 
d
i
r
e
c
t
i
o
n




`
x
f
s
_
r
e
p
a
i
r
`
 
p
h
a
s
e
 
5
 
r
e
b
u
i
l
d
s
 
e
a
c
h
 
g
r
o
u
p
 
h
e
a
d
e
r
 
a
n
d
 
i
t
s
 
t
r
e
e
s
.
 
 
R
u
n
 
a
g
a
i
n
s
t
 
a


c
o
p
y
 
o
f
 
`
x
f
s
v
4
.
i
m
g
`
 
i
t
 
m
o
v
e
d
,
 
f
o
r
 
g
r
o
u
p
 
1
,
 
`
b
t
r
e
e
b
l
k
s
`
 
5
8
 
→
 
7
2
,
 
`
f
r
e
e
b
l
k
s
`


1
0
7
2
9
 
→
 
1
0
7
1
1
 
a
n
d
 
t
h
e
 
w
i
n
d
o
w
 
`
(
8
5
,
 
9
0
,
 
6
)
`
 
→
 
`
(
0
,
 
7
,
 
8
)
`
;
 
f
o
r
 
g
r
o
u
p
 
3
,


`
b
t
r
e
e
b
l
k
s
`
 
2
6
7
 
→
 
3
5
0
,
 
`
f
r
e
e
b
l
k
s
`
 
2
3
8
6
8
 
→
 
2
3
7
7
3
 
a
n
d
 
t
h
e
 
w
i
n
d
o
w


`
(
2
6
,
 
3
3
,
 
8
)
`
 
→
 
`
(
0
,
 
1
9
,
 
2
0
)
`
.
 
 
T
w
o
 
t
h
i
n
g
s
 
f
o
l
l
o
w
,
 
a
n
d
 
o
n
l
y
 
t
w
o
:




*
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
r
i
s
e
s
 
w
i
t
h
 
t
h
e
 
n
u
m
b
e
r
 
o
f
 
n
o
n
-
r
o
o
t
 
b
l
o
c
k
s
 
t
h
e
 
t
r
e
e
s
 
a
c
q
u
i
r
e
,


 
 
a
n
d
 
t
h
e
 
t
w
o
 
f
r
e
s
h
 
r
o
o
t
 
b
l
o
c
k
s
 
a
r
e
 
n
o
t
 
c
o
u
n
t
e
d
 
—
 
w
h
i
c
h
 
i
s
 
t
h
e
 
m
e
a
s
u
r
e
d


 
 
d
e
f
i
n
i
t
i
o
n
 
a
b
o
v
e
,
 
c
o
n
f
i
r
m
e
d
 
b
y
 
t
h
e
 
p
a
r
t
y
 
t
h
a
t
 
w
r
o
t
e
 
t
h
e
m
.


*
 
`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
f
a
l
l
s
 
b
y
 
t
h
e
 
b
l
o
c
k
s
 
t
h
e
 
t
r
e
e
s
 
t
o
o
k
:
 
1
4
 
n
e
w
 
n
o
n
-
r
o
o
t
 
b
l
o
c
k
s
,


 
 
t
w
o
 
n
e
w
 
r
o
o
t
s
 
a
n
d
 
t
w
o
 
e
x
t
r
a
 
l
i
s
t
 
e
n
t
r
i
e
s
 
f
o
r
 
g
r
o
u
p
 
1
,
 
w
h
i
c
h
 
i
s
 
1
8
,
 
a
n
d
 
1
8
 
i
s


 
 
w
h
a
t
 
i
t
 
f
e
l
l
 
b
y
.
 
 
F
o
r
 
g
r
o
u
p
 
3
 
t
h
e
 
s
a
m
e
 
s
u
m
 
i
s
 
9
7
 
a
n
d
 
i
t
 
f
e
l
l
 
b
y
 
9
5
,
 
s
o
 
t
h
e


 
 
p
e
r
-
t
e
r
m
 
a
r
i
t
h
m
e
t
i
c
 
o
f
 
a
 
f
u
l
l
 
r
e
b
u
i
l
d
 
i
s
 
n
o
t
 
e
x
a
c
t
 
t
e
r
m
 
b
y
 
t
e
r
m
.
 
 
A
 
r
e
b
u
i
l
d


 
 
i
s
 
n
o
t
 
a
 
s
i
n
g
l
e
 
o
p
e
r
a
t
i
o
n
 
a
n
d
 
s
h
o
u
l
d
 
n
o
t
 
b
e
 
r
e
a
d
 
a
s
 
o
n
e
.




T
h
e
 
r
e
b
u
i
l
t
 
i
m
a
g
e
 
s
t
i
l
l
 
s
a
t
i
s
f
i
e
s
 
a
l
l
 
t
h
r
e
e
 
i
n
v
a
r
i
a
n
t
s
 
a
b
o
v
e
,
 
i
n
c
l
u
d
i
n
g
 
t
h
e


d
e
v
i
c
e
-
w
i
d
e
 
i
d
e
n
t
i
t
y
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
c
h
e
c
k
 
t
h
a
t
 
m
a
t
t
e
r
s
.




-
-
-




#
#
 
W
h
a
t
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
'
s
 
o
w
n
e
r
s
h
i
p
 
c
o
s
t
s




T
h
i
s
 
i
s
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
t
h
e
 
o
l
d
 
v
e
r
s
i
o
n
 
o
f
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
c
a
l
l
e
d
 
b
l
o
c
k
e
d
 
a
n
d
 
c
o
u
l
d


n
o
t
 
w
r
i
t
e
 
d
o
w
n
.
 
 
I
t
 
i
s
 
n
o
w
 
m
e
a
s
u
r
e
d
,
 
s
o
 
i
t
 
i
s
 
w
r
i
t
t
e
n
 
d
o
w
n
.




A
 
b
l
o
c
k
 
c
a
n
 
b
e
 
i
n
 
t
h
r
e
e
 
p
l
a
c
e
s
 
a
n
d
 
b
e
 
f
r
e
e
 
o
n
 
t
h
e
 
d
e
v
i
c
e
 
i
n
 
a
l
l
 
t
h
r
e
e
,
 
w
h
i
c
h
 
i
s


w
h
a
t
 
t
h
e
 
i
d
e
n
t
i
t
y
 
i
n


[
M
e
a
s
u
r
e
d
 
i
n
v
a
r
i
a
n
t
s
]
(
#
t
h
e
-
s
u
p
e
r
b
l
o
c
k
s
-
f
r
e
e
-
c
o
u
n
t
-
h
a
s
-
t
h
r
e
e
-
t
e
r
m
s
-
a
n
d
-
a
l
l
-
t
h
r
e
e
-
a
r
e
-
m
e
a
s
u
r
e
d
)


i
s
 
c
o
u
n
t
i
n
g
.
 
 
A
 
b
-
t
r
e
e
 
n
o
d
e
 
m
o
v
i
n
g
 
*
i
n
t
o
*
 
t
h
e
 
t
r
e
e
s
 
t
h
e
r
e
f
o
r
e
 
t
r
a
d
e
s
 
o
n
e
 
t
e
r
m
 
f
o
r


a
n
o
t
h
e
r
,
 
a
n
d
 
t
h
e
 
t
w
o
 
w
a
y
s
 
i
n
 
m
o
v
e
 
d
i
f
f
e
r
e
n
t
 
t
e
r
m
s
:




|
 
|
 
`
f
l
c
o
u
n
t
`
 
|
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
|
 
`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
|
 
`
s
b
_
f
d
b
l
o
c
k
s
`
 
|


|
:
-
-
|
:
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
t
a
k
e
n
 
o
f
f
 
t
h
e
 
f
r
e
e
 
l
i
s
t
 
|
 
−
1
 
|
 
*
*
+
1
*
*
 
|
 
u
n
c
h
a
n
g
e
d
 
|
 
u
n
c
h
a
n
g
e
d
 
|


|
 
t
a
k
e
n
 
o
u
t
 
o
f
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
|
 
u
n
c
h
a
n
g
e
d
 
|
 
*
*
+
1
*
*
 
|
 
−
1
 
|
 
u
n
c
h
a
n
g
e
d
 
|




`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
i
s
 
t
h
e
 
s
u
m
 
o
f
 
t
h
e
 
b
n
o
 
t
r
e
e
'
s
 
r
e
c
o
r
d
 
l
e
n
g
t
h
s
,
 
s
o
 
a
 
b
l
o
c
k
 
t
h
a
t
 
w
a
s


n
e
v
e
r
 
i
n
 
a
 
t
r
e
e
 
d
o
e
s
 
n
o
t
 
a
p
p
e
a
r
 
i
n
 
i
t
,
 
a
n
d
 
o
n
e
 
t
h
a
t
 
w
a
s
 
i
s
 
s
i
m
p
l
y
 
n
o
 
l
o
n
g
e
r


t
h
e
r
e
.
 
 
T
h
e
 
r
o
o
t
s
 
a
r
e
 
n
o
t
 
c
h
a
r
g
e
d
 
t
o
 
`
b
t
r
e
e
b
l
k
s
`
,
 
s
o
 
a
 
b
l
o
c
k
 
t
h
a
t
 
b
e
c
o
m
e
s
 
a
 
*
n
e
w


r
o
o
t
*
 
—
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
a
 
t
r
e
e
 
s
p
l
i
t
t
i
n
g
 
a
t
 
t
h
e
 
t
o
p
 
d
o
e
s
 
—
 
i
s
 
n
o
t
 
c
o
u
n
t
e
d
,
 
a
n
d
 
t
h
e


o
l
d
 
r
o
o
t
 
i
t
 
d
i
s
p
l
a
c
e
s
 
b
e
c
o
m
e
s
 
a
 
n
o
n
-
r
o
o
t
 
n
o
d
e
 
a
n
d
 
i
s
.




I
m
p
l
e
m
e
n
t
e
d
 
i
n
 
`
T
r
a
n
s
a
c
t
i
o
n
B
l
o
c
k
s
:
:
t
a
k
e
_
b
t
r
e
e
_
b
l
o
c
k
`
,
 
o
n
 
b
o
t
h
 
p
a
t
h
s
,
 
i
n
 
t
h
e
 
s
a
m
e


t
r
a
n
s
a
c
t
i
o
n
 
a
s
 
t
h
e
 
t
a
k
e
,
 
a
n
d
 
c
h
e
c
k
e
d
 
b
y
 
a
 
t
e
s
t
 
t
h
a
t
 
a
s
s
e
r
t
s
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
r
a
t
h
e
r


t
h
a
n
 
t
h
e
 
b
l
o
c
k
 
n
u
m
b
e
r
:




`
`
`
t
e
x
t


a
g
0
:
 
e
n
t
r
y
 
7
 
o
f
f
 
t
h
e
 
l
i
s
t
;
 
w
i
n
d
o
w
 
(
1
,
4
,
4
)
 
-
>
 
(
2
,
4
,
3
)
;


 
 
 
 
 
f
r
e
e
b
l
k
s
 
3
0
1
4
4
 
-
>
 
3
0
1
4
4
;
 
b
t
r
e
e
b
l
k
s
 
0
 
-
>
 
1
;
 
s
b
_
f
d
b
l
o
c
k
s
 
9
0
6
2
4
 
-
>
 
9
0
6
2
4


`
`
`




W
h
a
t
 
i
s
 
s
t
i
l
l
 
n
o
t
 
e
s
t
a
b
l
i
s
h
e
d
 
i
s
 
t
h
e
 
*
o
r
d
e
r
i
n
g
*
 
w
i
t
h
i
n
 
a
 
s
i
n
g
l
e
 
o
p
e
r
a
t
i
o
n
 
—
 
w
h
i
c
h


o
f
 
t
h
e
s
e
 
h
a
s
 
t
o
 
b
e
 
o
n
 
t
h
e
 
i
m
a
g
e
 
b
e
f
o
r
e
 
t
h
e
 
o
p
e
r
a
t
i
o
n
 
t
h
a
t
 
f
o
l
l
o
w
s
 
i
t
 
c
a
n
 
s
e
e
 
i
t
 
—


b
e
c
a
u
s
e
 
t
h
e
 
d
e
v
e
l
o
p
m
e
n
t
 
e
n
v
i
r
o
n
m
e
n
t
 
c
a
n
n
o
t
 
m
o
u
n
t
 
a
n
 
X
F
S
 
i
m
a
g
e
 
a
n
d
 
s
o
 
c
a
n
n
o
t


p
r
o
v
o
k
e
 
a
n
d
 
w
a
t
c
h
 
o
n
e
 
i
n
c
r
e
m
e
n
t
a
l
 
o
p
e
r
a
t
i
o
n
.
 
 
W
h
a
t
 
i
s
 
e
s
t
a
b
l
i
s
h
e
d
 
i
s
 
t
h
a
t
 
t
h
e


s
t
a
t
e
 
e
i
t
h
e
r
 
e
n
d
 
o
f
 
t
h
e
 
o
p
e
r
a
t
i
o
n
 
i
s
 
a
 
s
t
a
t
e
 
n
a
t
i
v
e
 
X
F
S
 
p
r
o
d
u
c
e
s
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t


`
x
f
s
_
r
e
p
a
i
r
`
 
a
n
d
 
t
h
e
 
i
d
e
n
t
i
t
y
 
b
o
t
h
 
a
g
r
e
e
 
o
n
.




T
h
e
 
r
e
v
e
r
s
e
 
i
s
 
i
n
 
t
h
e
 
s
a
m
e
 
t
a
b
l
e
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
 
o
n
e
 
w
i
t
h
 
a
 
c
h
o
i
c
e
 
i
n
 
i
t
:




|
 
a
 
n
o
d
e
 
i
s
 
r
e
l
e
a
s
e
d
 
t
o
 
|
 
`
f
l
c
o
u
n
t
`
 
|
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
|
 
`
a
g
f
_
f
r
e
e
b
l
k
s
`
 
|
 
`
s
b
_
f
d
b
l
o
c
k
s
`
 
|


|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
t
h
e
 
f
r
e
e
 
l
i
s
t
 
|
 
+
1
 
|
 
*
*
−
1
*
*
 
|
 
u
n
c
h
a
n
g
e
d
 
|
 
u
n
c
h
a
n
g
e
d
 
|


|
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
,
 
b
e
c
a
u
s
e
 
t
h
e
 
l
i
s
t
 
i
s
 
f
u
l
l
 
|
 
u
n
c
h
a
n
g
e
d
 
|
 
*
*
−
1
*
*
 
|
 
+
1
 
|
 
u
n
c
h
a
n
g
e
d
 
|




`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
c
o
m
e
s
 
d
o
w
n
 
e
i
t
h
e
r
 
w
a
y
,
 
b
e
c
a
u
s
e
 
e
i
t
h
e
r
 
w
a
y
 
t
h
e
 
b
l
o
c
k
 
i
s
 
n
o
 
l
o
n
g
e
r


o
n
e
 
t
h
e
 
t
r
e
e
s
 
h
o
l
d
.
 
 
W
h
i
c
h
 
o
f
 
t
h
e
 
o
t
h
e
r
 
t
w
o
 
m
o
v
e
s
 
i
s
 
n
o
t
 
a
 
p
r
e
f
e
r
e
n
c
e
 
—
 
i
t
 
i
s


w
h
e
t
h
e
r
 
t
h
e
 
l
i
s
t
 
h
a
d
 
r
o
o
m
,
 
a
n
d
 
t
h
e
 
f
r
e
e
 
l
i
s
t
 
i
s
 
n
o
t
 
a
n
 
i
n
f
i
n
i
t
e
 
q
u
e
u
e
.




#
#
#
 
A
 
f
u
l
l
 
f
r
e
e
 
l
i
s
t
 
i
s
 
r
e
a
c
h
a
b
l
e
 
o
n
l
y
 
b
y
 
g
i
v
i
n
g
 
b
a
c
k




W
o
r
t
h
 
w
r
i
t
i
n
g
 
d
o
w
n
 
b
e
c
a
u
s
e
 
i
t
 
i
s
 
n
o
t
 
o
b
v
i
o
u
s
 
a
n
d
 
i
t
 
m
a
k
e
s
 
a
 
w
h
o
l
e
 
b
r
a
n
c
h
 
o
f
 
t
h
e


c
o
d
e
 
n
e
a
r
l
y
 
d
e
a
d
.




`
a
p
p
e
n
d
_
t
o
_
t
h
e
_
f
r
e
e
_
l
i
s
t
`
 
d
e
l
i
b
e
r
a
t
e
l
y
 
k
e
e
p
s
 
a
 
s
l
o
t
 
i
n
 
h
a
n
d
:
 
i
t
 
s
t
o
c
k
s
 
t
h
e
 
l
i
s
t


w
i
t
h
 
b
l
o
c
k
s
 
b
e
i
n
g
 
f
r
e
e
d
 
a
n
d
 
s
t
o
p
s
 
o
n
e
 
s
h
o
r
t
 
o
f
 
t
h
e
 
a
r
r
a
y
'
s
 
e
n
d
.
 
 
S
o
 
a
 
l
i
s
t


s
t
o
c
k
e
d
 
o
n
l
y
 
b
y
 
f
r
e
e
i
n
g
 
n
e
v
e
r
 
f
i
l
l
s
 
—
 
t
h
e
 
w
i
n
d
o
w
'
s
 
`
l
a
s
t
`
 
s
t
o
p
s
 
a
t
 
1
2
6
 
i
n
 
a


1
2
8
-
s
l
o
t
 
a
r
r
a
y
 
—
 
a
n
d
 
`
A
g
f
l
:
:
g
i
v
e
_
b
a
c
k
`
,
 
w
h
i
c
h
 
r
e
f
u
s
e
s
 
a
t
 
1
2
7
,
 
h
a
s
 
a
 
s
l
o
t
 
t
o
 
s
p
a
r
e


f
o
r
 
e
v
e
r
.
 
 
M
e
a
s
u
r
e
d
 
o
n
 
`
x
f
s
v
4
.
i
m
g
`
:
 
a
f
t
e
r
 
s
t
o
c
k
i
n
g
,
 
t
h
r
e
e
 
t
a
k
e
-
a
n
d
-
g
i
v
e
-
b
a
c
k


r
o
u
n
d
 
t
r
i
p
s
 
a
g
a
i
n
s
t
 
t
h
a
t
 
l
i
s
t
 
g
o




`
`
`
t
e
x
t


T
o
T
h
e
F
r
e
e
L
i
s
t
,
 
T
o
F
r
e
e
S
p
a
c
e
,
 
T
o
F
r
e
e
S
p
a
c
e


`
`
`




s
o
 
t
h
e
 
l
i
s
t
 
t
a
k
e
s
 
t
h
e
 
o
n
e
 
s
l
o
t
 
s
t
o
c
k
i
n
g
 
k
e
p
t
 
i
n
 
h
a
n
d
 
a
n
d
 
t
h
e
n
 
h
a
s
 
n
o
n
e
 
l
e
f
t
.
 
 
T
h
e


f
a
l
l
b
a
c
k
 
b
r
a
n
c
h
 
i
s
 
a
l
i
v
e
,
 
b
u
t
 
o
n
l
y
 
b
e
c
a
u
s
e
 
o
f
 
t
h
e
 
o
p
e
r
a
t
i
o
n
 
i
t
 
b
e
l
o
n
g
s
 
t
o
,
 
a
n
d


a
n
y
t
h
i
n
g
 
t
h
a
t
 
s
t
o
c
k
s
 
t
h
e
 
l
i
s
t
 
m
o
r
e
 
t
i
g
h
t
l
y
 
t
h
a
n
 
`
f
r
e
e
_
i
n
_
g
r
o
u
p
`
 
d
o
e
s
 
w
o
u
l
d
 
m
a
k
e


i
t
 
u
n
r
e
a
c
h
a
b
l
e
.




-
-
-




#
#
 
N
e
x
t
 
w
o
r
k




T
h
e
 
d
e
p
e
n
d
e
n
c
y
 
o
r
d
e
r
 
b
e
l
o
w
 
i
s
 
d
e
l
i
b
e
r
a
t
e
,
 
a
n
d
 
t
h
e
 
r
u
l
e
 
i
s
 
t
h
a
t
 
a
 
l
a
t
e
r
 
f
e
a
t
u
r
e


w
h
i
c
h
 
e
x
p
o
s
e
s
 
a
 
m
i
s
s
i
n
g
 
e
a
r
l
i
e
r
 
i
n
v
a
r
i
a
n
t
 
s
t
o
p
s
 
a
n
d
 
t
h
e
 
e
a
r
l
i
e
r
 
i
n
v
a
r
i
a
n
t
 
i
s


f
i
x
e
d
 
f
i
r
s
t
.
 
 
A
 
w
o
r
k
a
r
o
u
n
d
 
i
s
 
n
o
t
 
a
 
f
i
x
.




`
`
`
t
e
x
t


A
G
F
L
 
c
o
r
r
e
c
t
n
e
s
s


 
 
 
 
|


 
 
 
 
v


m
e
t
a
d
a
t
a
-
b
l
o
c
k
 
o
w
n
e
r
s
h
i
p
 
a
n
d
 
a
c
c
o
u
n
t
i
n
g


 
 
 
 
|


 
 
 
 
v


f
r
e
e
-
s
p
a
c
e
 
t
r
e
e
 
s
t
r
u
c
t
u
r
a
l
 
m
u
t
a
t
i
o
n


 
 
 
 
|


 
 
 
 
v


n
e
w
 
i
n
o
d
e
 
c
h
u
n
k
s


 
 
 
 
|


 
 
 
 
v


f
i
l
e
 
e
x
t
e
n
s
i
o
n


 
 
 
 
|


 
 
 
 
v


B
M
B
T
 
g
r
o
w
t
h


 
 
 
 
|


 
 
 
 
v


t
r
u
n
c
a
t
e
 
a
n
d
 
f
r
e
e


 
 
 
 
|


 
 
 
 
v


d
i
r
e
c
t
o
r
i
e
s
 
a
n
d
 
n
a
m
e
s
p
a
c
e


 
 
 
 
|


 
 
 
 
v


f
u
l
l
 
w
r
i
t
e
 
s
u
p
p
o
r
t


`
`
`




C
o
n
c
r
e
t
e
l
y
,
 
i
n
 
o
r
d
e
r
:




1
.
 
~
~
*
*
A
G
F
L
 
l
i
v
e
-
w
i
n
d
o
w
 
c
o
r
r
e
c
t
n
e
s
s
.
*
*
~
~
 
 
D
o
n
e
:
 
`
f
l
c
o
u
n
t
 
=
=
 
0
`
 
i
s
 
a
n
 
o
r
d
i
n
a
r
y


 
 
 
a
l
l
o
c
a
t
o
r
 
c
o
n
d
i
t
i
o
n
 
a
n
d
 
t
h
e
 
b
l
o
c
k
 
c
o
m
e
s
 
f
r
o
m
 
t
h
e
 
g
r
o
u
p
'
s
 
o
w
n
 
f
r
e
e
 
s
p
a
c
e
;


 
 
 
`
f
l
c
o
u
n
t
 
>
 
0
`
 
w
i
t
h
 
a
 
s
l
o
t
 
i
n
 
t
h
e
 
l
i
v
e
 
w
i
n
d
o
w
 
t
h
a
t
 
i
s
 
n
u
l
l
,
 
z
e
r
o
,
 
o
r
 
o
u
t
 
o
f


 
 
 
t
h
e
 
a
r
r
a
y
 
i
s
 
a
 
*
*
c
o
r
r
u
p
t
*
*
 
g
r
o
u
p
 
a
n
d
 
s
a
y
s
 
s
o
;
 
n
o
t
h
i
n
g
 
o
u
t
s
i
d
e
 
t
h
e
 
w
i
n
d
o
w
 
i
s


 
 
 
r
e
a
d
.




2
.
 
~
~
*
*
T
h
e
 
A
G
F
L
 
e
m
p
t
y
-
w
i
n
d
o
w
 
r
e
g
r
e
s
s
i
o
n
 
t
e
s
t
.
*
*
~
~
 
 
D
o
n
e
,
 
o
n
 
a
 
r
e
a
l
 
i
m
a
g
e
,
 
a
n
d
 
n
o
w


 
 
 
a
 
r
o
u
n
d
 
t
r
i
p
:
 
t
w
o
 
b
l
o
c
k
s
 
t
a
k
e
n
 
o
u
t
 
o
f
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
,
 
w
r
i
t
t
e
n
 
a
s
 
n
o
d
e
s
,


 
 
 
a
n
d
 
g
i
v
e
n
 
b
a
c
k
,
 
w
i
t
h
 
t
h
e
 
t
r
e
e
s
,
 
t
h
e
 
l
i
s
t
 
a
n
d
 
t
h
e
 
i
d
e
n
t
i
t
y
 
a
l
l
 
c
h
e
c
k
e
d
.
 
 
T
h
e


 
 
 
r
e
p
a
i
r
 
c
h
e
c
k
 
i
s
 
t
h
e
 
o
n
e
 
c
l
a
i
m
 
s
t
i
l
l
 
o
u
t
s
t
a
n
d
i
n
g
,
 
a
n
d
 
i
t
 
i
s
 
o
u
t
s
t
a
n
d
i
n
g
 
f
o
r
 
a


 
 
 
r
e
a
s
o
n
 
t
h
a
t
 
t
u
r
n
e
d
 
o
u
t
 
t
o
 
b
e
 
a
b
o
u
t
 
t
h
e
 
e
v
i
d
e
n
c
e
 
r
a
t
h
e
r
 
t
h
a
n
 
a
b
o
u
t
 
t
h
e
 
c
o
d
e
 
—


 
 
 
s
e
e
 
[
T
h
e
 
A
G
F
L
 
→
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
 
q
u
e
s
t
i
o
n
]
(
#
t
h
e
-
a
g
f
l
-
-
o
r
d
i
n
a
r
y
-
f
r
e
e
-
s
p
a
c
e
-
q
u
e
s
t
i
o
n
-
a
n
s
w
e
r
e
d
)
.


 
 
 
S
h
o
r
t
 
v
e
r
s
i
o
n
:
 
c
l
e
a
r
i
n
g
 
a
 
s
t
o
c
k
e
d
 
l
i
s
t
'
s
 
w
i
n
d
o
w
 
o
r
p
h
a
n
s
 
t
h
e
 
b
l
o
c
k
s
 
i
t
 
w
a
s


 
 
 
h
o
l
d
i
n
g
,
 
a
n
d
 
t
h
e
r
e
 
i
s
 
n
o
 
*
e
s
t
a
b
l
i
s
h
e
d
*
 
o
p
e
r
a
t
i
o
n
 
t
h
a
t
 
p
u
t
s
 
t
h
e
m
 
b
a
c
k
,
 
s
o
 
t
h
e
r
e


 
 
 
i
s
 
n
o
 
h
o
n
e
s
t
 
w
a
y
 
t
o
 
b
u
i
l
d
 
t
h
e
 
s
t
a
t
e
 
o
n
 
a
n
 
i
m
a
g
e
 
w
h
o
s
e
 
l
i
s
t
 
h
a
d
 
s
o
m
e
t
h
i
n
g
 
i
n


 
 
 
i
t
.
 
 
T
h
e
 
g
a
p
 
i
s
 
a
s
s
e
r
t
e
d
 
e
x
a
c
t
l
y
 
(
`
s
b
_
f
d
b
l
o
c
k
s
 
−
 
2
`
,
 
n
o
t
 
`
−
 
4
`
)
 
s
o
 
n
e
i
t
h
e
r
 
o
f


 
 
 
t
h
e
 
t
w
o
 
e
r
r
o
r
s
 
h
i
d
e
s
 
b
e
h
i
n
d
 
t
h
e
 
o
t
h
e
r
.




3
.
 
~
~
*
*
T
h
e
 
m
e
a
s
u
r
e
d
 
A
G
F
L
 
→
 
b
-
t
r
e
e
 
t
r
a
n
s
i
t
i
o
n
.
*
*
~
~
 
 
D
o
n
e
,
 
f
o
r
 
b
o
t
h
 
o
f
 
t
h
e
 
p
l
a
c
e
s


 
 
 
a
 
b
l
o
c
k
 
c
a
n
 
c
o
m
e
 
f
r
o
m
,
 
w
i
t
h
 
t
h
e
 
c
o
u
n
t
e
r
s
 
a
s
s
e
r
t
e
d
.
 
 
S
e
e


 
 
 
[
W
h
a
t
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
'
s
 
o
w
n
e
r
s
h
i
p
 
c
o
s
t
s
]
(
#
w
h
a
t
-
a
-
m
e
t
a
d
a
t
a
-
b
l
o
c
k
s
-
o
w
n
e
r
s
h
i
p
-
c
o
s
t
s
)
.




4
.
 
~
~
*
*
T
h
e
 
r
e
v
e
r
s
e
 
t
r
a
n
s
i
t
i
o
n
,
 
b
-
t
r
e
e
 
→
 
A
G
F
L
,
 
a
n
d
 
t
h
e
 
A
G
F
L
-
f
u
l
l
 
c
a
s
e
.
*
*
~
~
 
 
D
o
n
e


 
 
 
f
o
r
 
t
h
e
 
a
c
c
o
u
n
t
i
n
g
,
 
b
o
t
h
 
b
r
a
n
c
h
e
s
,
 
c
h
e
c
k
e
d
 
o
n
 
a
 
r
e
a
l
 
i
m
a
g
e
 
w
i
t
h
 
`
x
f
s
_
r
e
p
a
i
r
`


 
 
 
a
c
c
e
p
t
i
n
g
 
t
h
e
 
r
e
s
u
l
t
.
 
 
W
h
a
t
 
i
s
 
s
t
i
l
l
 
m
i
s
s
i
n
g
 
i
s
 
t
h
e
 
c
a
l
l
e
r
:
 
n
o
t
h
i
n
g
 
r
e
l
e
a
s
e
s
 
a


 
 
 
n
o
d
e
,
 
b
e
c
a
u
s
e
 
n
o
t
h
i
n
g
 
m
e
r
g
e
s
 
a
 
l
e
a
f
,
 
b
e
c
a
u
s
e
 
n
o
t
h
i
n
g
 
c
a
n
 
r
e
a
c
h
 
a
 
l
e
a
f
 
t
h
a
t


 
 
 
n
e
e
d
s
 
m
e
r
g
i
n
g
.




5
.
 
~
~
*
*
T
h
e
 
f
r
e
e
-
s
p
a
c
e
 
c
o
n
s
i
s
t
e
n
c
y
 
o
r
a
c
l
e
.
*
*
~
~
 
 
D
o
n
e
 
f
o
r
 
t
h
e
 
r
e
l
a
t
i
o
n
s
h
i
p
s
 
t
h
a
t


 
 
 
e
x
i
s
t
.
 
 
O
n
e
 
c
h
e
c
k
,
 
a
s
k
e
d
 
o
f
 
e
v
e
r
y
 
g
r
o
u
p
 
o
f
 
e
v
e
r
y
 
u
n
p
a
c
k
e
d
 
i
m
a
g
e
 
a
n
d
 
a
g
a
i
n


 
 
 
a
f
t
e
r
 
e
a
c
h
 
s
t
e
p
 
o
f
 
a
 
s
e
q
u
e
n
c
e
 
o
f
 
o
p
e
r
a
t
i
o
n
s
 
t
h
a
t
 
t
o
u
c
h
 
a
l
l
 
o
f
 
i
t
:
 
t
h
e
 
t
w
o


 
 
 
t
r
e
e
s
 
h
o
l
d
 
t
h
e
 
s
a
m
e
 
f
r
e
e
 
s
p
a
c
e
,
 
n
o
 
t
w
o
 
r
u
n
s
 
o
v
e
r
l
a
p
 
o
r
 
t
o
u
c
h
,
 
t
h
e
 
t
o
t
a
l
s
 
a
n
d


 
 
 
t
h
e
 
l
o
n
g
e
s
t
 
r
u
n
 
m
a
t
c
h
 
t
h
e
 
h
e
a
d
e
r
,
 
t
h
e
 
f
r
e
e
 
l
i
s
t
'
s
 
e
n
t
r
i
e
s
 
a
r
e
 
v
a
l
i
d
 
a
n
d
 
a
r
e


 
 
 
n
o
t
 
a
l
s
o
 
f
r
e
e
 
s
p
a
c
e
,
 
a
n
d
 
t
h
e
 
d
e
v
i
c
e
-
w
i
d
e
 
i
d
e
n
t
i
t
y
 
h
o
l
d
s
.
 
 
W
h
a
t
 
w
r
i
t
i
n
g
 
t
h
a
t


 
 
 
f
o
u
n
d
 
i
s
 
i
n
 
[
W
h
a
t
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
'
s
 
o
w
n
e
r
s
h
i
p
 
c
o
s
t
s
]
(
#
w
h
a
t
-
a
-
m
e
t
a
d
a
t
a
-
b
l
o
c
k
s
-
o
w
n
e
r
s
h
i
p
-
c
o
s
t
s
)
:


 
 
 
a
 
b
l
o
c
k
 
t
h
a
t
 
h
a
s
 
b
e
e
n
 
t
a
k
e
n
 
f
o
r
 
a
 
n
o
d
e
 
b
u
t
 
n
o
t
 
y
e
t
 
l
i
n
k
e
d
 
i
n
t
o
 
a
 
t
r
e
e
 
i
s
 
a


 
 
 
s
t
a
t
e
 
`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
f
u
s
e
s
,
 
s
o
 
"
c
h
a
r
g
e
d
 
f
o
r
"
 
a
n
d
 
"
h
e
l
d
 
b
y
 
a
 
t
r
e
e
"
 
h
a
v
e
 
t
o
 
m
o
v
e


 
 
 
i
n
 
t
h
e
 
s
a
m
e
 
t
r
a
n
s
a
c
t
i
o
n
,
 
n
o
t
 
o
n
e
 
a
f
t
e
r
 
t
h
e
 
o
t
h
e
r
.


6
.
 
~
~
*
*
T
r
e
e
 
b
a
l
a
n
c
i
n
g
.
*
*
~
~
 
 
D
o
n
e
 
f
o
r
 
t
h
e
 
c
a
s
e
 
t
h
a
t
 
a
r
i
s
e
s
.
 
 
T
h
e
 
o
c
c
u
p
a
n
c
y
 
r
u
l
e
 
i
s


 
 
 
m
e
a
s
u
r
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
a
s
s
u
m
e
d
,
 
b
y
 
s
h
r
i
n
k
i
n
g
 
a
 
r
e
a
l
 
l
e
a
f
 
a
n
d
 
a
s
k
i
n
g


 
 
 
`
x
f
s
_
r
e
p
a
i
r
`
:




 
 
 
`
`
`
t
e
x
t


 
 
 
g
r
o
u
p
 
1
'
s
 
f
i
r
s
t
 
b
n
o
 
l
e
a
f
,
 
3
1
 
r
e
c
o
r
d
s
,
 
s
e
t
 
t
o
 
3
0
:


 
 
 
 
 
 
 
b
a
d
 
b
t
r
e
e
 
n
r
e
c
s
 
(
3
0
,
 
m
i
n
=
3
1
,
 
m
a
x
=
6
2
)
 
i
n
 
b
t
b
n
o
 
b
l
o
c
k
 
1
/
4


 
 
 
`
`
`




 
 
 
S
o
 
a
 
l
e
a
f
 
i
n
 
a
 
5
1
2
-
b
y
t
e
 
b
l
o
c
k
 
h
o
l
d
s
 
a
t
 
m
o
s
t
 
6
2
 
r
e
c
o
r
d
s
 
a
n
d
,
 
*
*
i
f
 
i
t
 
h
a
s
 
a


 
 
 
p
a
r
e
n
t
*
*
,
 
a
t
 
l
e
a
s
t
 
3
1
 
—
 
h
a
l
f
 
o
f
 
6
2
,
 
r
o
u
n
d
e
d
 
u
p
.
 
 
A
 
d
e
l
e
t
e
 
t
h
a
t
 
l
e
a
v
e
s
 
a


 
 
 
n
o
n
-
r
o
o
t
 
l
e
a
f
 
b
e
l
o
w
 
t
h
a
t
 
h
a
s
 
t
o
 
m
e
r
g
e
,
 
w
h
a
t
e
v
e
r
 
t
h
e
 
B
+
t
r
e
e
 
d
o
c
u
m
e
n
t
a
t
i
o
n
'
s


 
 
 
"
s
h
o
u
l
d
"
 
s
a
y
s
.




 
 
 
A
n
d
 
t
h
e
 
h
a
l
f
 
t
h
a
t
 
b
o
u
n
d
s
 
i
t
:
 
*
*
a
 
l
e
a
f
 
t
h
a
t
 
i
s
 
a
l
s
o
 
t
h
e
 
r
o
o
t
 
i
s
 
e
x
e
m
p
t
.
*
*


 
 
 
G
r
o
u
p
 
0
'
s
 
t
w
o
 
t
r
e
e
s
 
a
r
e
 
s
i
n
g
l
e
 
l
e
a
v
e
s
,
 
a
n
d
 
w
i
t
h
 
b
o
t
h
 
s
h
r
u
n
k
 
t
o
g
e
t
h
e
r
 
—
 
s
o
 
t
h
a
t


 
 
 
t
h
e
y
 
s
t
i
l
l
 
a
g
r
e
e
,
 
a
n
d
 
w
i
t
h
 
t
h
e
 
l
e
f
t
o
v
e
r
 
r
e
c
o
r
d
 
s
l
o
t
s
 
c
l
e
a
r
e
d
 
s
o
 
t
h
a
t
 
o
c
c
u
p
a
n
c
y


 
 
 
i
s
 
t
h
e
 
o
n
l
y
 
t
h
i
n
g
 
w
r
o
n
g
 
—
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
s
a
y
s
 
n
o
t
h
i
n
g
 
a
b
o
u
t
 
t
h
e
 
n
u
m
b
e
r
 
o
f


 
 
 
r
e
c
o
r
d
s
,
 
a
t
 
1
0
 
r
e
c
o
r
d
s
,
 
a
t
 
3
,
 
o
r
 
a
t
 
1
.
 
 
S
o
 
a
 
t
r
e
e
 
t
h
a
t
 
i
s
 
a
 
s
i
n
g
l
e
 
l
e
a
f
 
n
e
v
e
r


 
 
 
n
e
e
d
s
 
a
 
m
e
r
g
e
,
 
a
n
d
 
a
 
m
e
r
g
e
 
t
h
a
t
 
e
m
p
t
i
e
s
 
a
 
w
h
o
l
e
 
i
n
t
e
r
i
o
r
 
n
o
d
e
 
s
t
o
p
s
 
a
t
 
t
h
e
 
r
o
o
t


 
 
 
r
a
t
h
e
r
 
t
h
a
n
 
c
o
l
l
a
p
s
i
n
g
 
i
t
.




 
 
 
T
h
e
 
f
i
r
s
t
 
a
t
t
e
m
p
t
 
a
t
 
t
h
a
t
 
e
x
p
e
r
i
m
e
n
t
 
s
h
r
a
n
k
 
t
h
e
 
r
o
o
t
 
l
e
a
v
e
s
 
w
i
t
h
o
u
t
 
c
l
e
a
r
i
n
g


 
 
 
t
h
e
 
l
e
f
t
o
v
e
r
s
 
a
n
d
 
r
e
p
o
r
t
e
d
 
t
h
e
 
r
o
o
t
 
a
s
 
c
o
n
s
t
r
a
i
n
e
d
;
 
i
t
 
w
a
s
 
n
o
t
.
 
 
R
e
p
a
i
r
 
h
a
d


 
 
 
s
t
o
p
p
e
d
 
o
n
 
t
h
e
 
t
r
e
e
s
 
d
i
s
a
g
r
e
e
i
n
g
.
 
 
A
n
 
a
b
s
e
n
t
 
c
o
m
p
l
a
i
n
t
 
m
e
a
n
s
 
n
o
t
h
i
n
g
 
u
n
l
e
s
s


 
 
 
r
e
p
a
i
r
 
g
o
t
 
f
a
r
 
e
n
o
u
g
h
 
t
o
 
h
a
v
e
 
m
a
d
e
 
o
n
e
,
 
s
o
 
t
h
e
 
t
e
s
t
 
a
l
s
o
 
a
s
s
e
r
t
s
 
t
h
a
t
 
t
h
e


 
 
 
a
c
c
o
u
n
t
i
n
g
 
c
o
m
p
l
a
i
n
t
 
*
i
s
*
 
p
r
e
s
e
n
t
 
i
n
 
t
h
e
 
p
a
t
c
h
e
d
 
i
m
a
g
e
.




 
 
 
W
i
t
h
 
t
h
e
 
r
u
l
e
 
m
e
a
s
u
r
e
d
,
 
t
h
e
 
m
e
r
g
e
 
i
t
s
e
l
f
 
t
u
r
n
e
d
 
o
u
t
 
t
o
 
b
e
 
*
*
a
l
r
e
a
d
y
 
w
r
i
t
t
e
n
*
*


 
 
 
—
 
`
w
a
l
k
_
u
p
`
 
m
e
r
g
e
s
 
a
 
s
h
o
r
t
 
l
e
a
f
 
i
n
t
o
 
i
t
s
 
l
e
f
t
 
s
i
b
l
i
n
g
,
 
s
h
a
r
e
s
 
r
e
c
o
r
d
s
 
a
c
r
o
s
s
 
t
h
e


 
 
 
b
o
u
n
d
a
r
y
 
w
h
e
n
 
t
h
e
 
s
i
b
l
i
n
g
 
i
s
 
f
u
l
l
,
 
u
n
l
i
n
k
s
 
t
h
e
 
n
o
d
e
 
a
n
d
 
r
e
l
i
n
k
s
 
i
t
s
 
t
w
o


 
 
 
n
e
i
g
h
b
o
u
r
s
 
—
 
a
n
d
 
a
l
r
e
a
d
y
 
f
u
z
z
e
d
 
i
n
 
m
e
m
o
r
y
,
 
3
0
0
 
r
a
n
d
o
m
i
s
e
d
 
t
a
k
e
-
a
n
d
-
f
r
e
e
 
r
o
u
n
d
s


 
 
 
p
e
r
 
s
e
e
d
 
a
g
a
i
n
s
t
 
a
 
m
o
d
e
l
 
o
f
 
w
h
i
c
h
 
b
l
o
c
k
s
 
a
r
e
 
f
r
e
e
.
 
 
W
h
a
t
 
w
a
s
 
m
i
s
s
i
n
g
 
w
a
s
 
n
o
t
 
t
h
e


 
 
 
t
r
e
e
 
w
o
r
k
 
b
u
t
 
t
h
e
 
t
w
o
 
t
h
i
n
g
s
 
o
n
l
y
 
a
 
r
e
a
l
 
i
m
a
g
e
 
c
o
u
l
d
 
s
h
o
w
,
 
a
n
d
 
b
o
t
h
 
t
u
r
n
e
d
 
u
p


 
 
 
t
h
e
 
m
o
m
e
n
t
 
a
 
m
e
r
g
e
 
w
a
s
 
a
c
t
u
a
l
l
y
 
r
u
n
 
o
n
 
o
n
e
:




 
 
 
*
 
*
*
t
h
e
 
g
r
o
u
p
 
k
e
p
t
 
c
h
a
r
g
i
n
g
 
f
o
r
 
t
h
e
 
n
o
d
e
 
t
h
e
 
m
e
r
g
e
 
r
e
l
e
a
s
e
d
.
*
*
 
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`


 
 
 
 
 
i
s
 
a
 
c
o
u
n
t
 
o
f
 
t
h
e
 
b
l
o
c
k
s
 
t
h
e
 
t
r
e
e
s
 
h
o
l
d
,
 
s
o
 
a
 
h
e
a
d
e
r
 
s
t
i
l
l
 
c
o
u
n
t
i
n
g
 
a
 
m
e
r
g
e
d
-


 
 
 
 
 
a
w
a
y
 
l
e
a
f
 
d
e
s
c
r
i
b
e
s
 
a
 
t
r
e
e
 
w
i
t
h
 
a
 
n
o
d
e
 
n
o
 
w
a
l
k
 
w
i
l
l
 
e
v
e
r
 
r
e
a
c
h
.
 
 
A
 
m
e
r
g
e
 
i
s


 
 
 
 
 
n
o
w
 
t
h
e
 
f
i
r
s
t
 
r
e
a
l
 
c
a
l
l
e
r
 
o
f
 
t
h
e
 
r
e
l
e
a
s
e
 
p
a
t
h
,
 
a
n
d
 
t
h
e
 
r
e
l
e
a
s
e
 
i
s
 
a
s
k
e
d
 
o
f
 
t
h
e


 
 
 
 
 
g
r
o
u
p
 
r
a
t
h
e
r
 
t
h
a
n
 
d
e
c
i
d
e
d
 
b
y
 
t
h
e
 
t
r
e
e
,
 
b
e
c
a
u
s
e
 
w
h
e
r
e
 
a
 
r
e
l
e
a
s
e
d
 
b
l
o
c
k
 
g
o
e
s


 
 
 
 
 
d
e
p
e
n
d
s
 
o
n
 
t
h
e
 
g
r
o
u
p
'
s
 
f
r
e
e
 
l
i
s
t
 
a
n
d
 
t
h
e
 
g
r
o
u
p
 
i
s
 
w
h
a
t
 
h
o
l
d
s
 
t
h
e
 
a
n
s
w
e
r
.




 
 
 
*
 
*
*
t
h
e
 
t
a
k
e
 
p
a
t
h
 
w
r
o
t
e
 
a
 
s
t
a
l
e
 
h
e
a
d
e
r
 
o
v
e
r
 
t
h
e
 
t
o
p
.
*
*
 
 
`
a
l
l
o
c
a
t
e
_
i
n
_
g
r
o
u
p
`


 
 
 
 
 
r
e
a
s
o
n
e
d
 
t
h
a
t
 
a
 
t
a
k
e
 
c
a
n
n
o
t
 
s
p
l
i
t
 
a
n
d
 
t
h
e
r
e
f
o
r
e
 
c
a
n
n
o
t
 
t
o
u
c
h
 
t
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r


 
 
 
 
 
—
 
w
h
i
c
h
 
w
a
s
 
t
r
u
e
 
w
h
i
l
e
 
t
a
k
i
n
g
 
c
o
u
l
d
 
o
n
l
y
 
s
h
r
i
n
k
 
l
e
a
v
e
s
,
 
a
n
d
 
s
t
o
p
p
e
d
 
b
e
i
n
g
 
t
r
u
e


 
 
 
 
 
t
h
e
 
m
o
m
e
n
t
 
a
 
s
h
r
i
n
k
 
c
o
u
l
d
 
m
e
r
g
e
 
o
n
e
.
 
 
I
t
 
n
o
w
 
r
e
a
d
s
 
t
h
e
 
h
e
a
d
e
r
 
a
g
a
i
n
 
b
e
f
o
r
e


 
 
 
 
 
w
r
i
t
i
n
g
 
i
t
,
 
e
x
a
c
t
l
y
 
a
s
 
t
h
e
 
f
r
e
e
 
p
a
t
h
 
a
l
r
e
a
d
y
 
h
a
d
 
t
o
.
 
 
T
h
a
t
 
i
s
 
t
h
e
 
t
h
i
r
d
 
t
i
m
e


 
 
 
 
 
t
h
i
s
 
s
u
i
t
e
 
h
a
s
 
f
o
u
n
d
 
t
h
a
t
 
s
a
m
e
 
b
u
g
 
i
n
 
a
 
d
i
f
f
e
r
e
n
t
 
p
l
a
c
e
:
 
t
h
e
 
h
e
a
d
e
r
 
i
s
 
s
h
a
r
e
d


 
 
 
 
 
b
y
 
e
v
e
r
y
t
h
i
n
g
 
t
h
a
t
 
t
o
u
c
h
e
s
 
a
 
g
r
o
u
p
,
 
s
o
 
a
 
c
o
p
y
 
t
a
k
e
n
 
b
e
f
o
r
e
 
t
h
e
 
w
o
r
k
 
i
s
 
a
 
c
o
p
y


 
 
 
 
 
t
a
k
e
n
 
t
o
o
 
e
a
r
l
y
.




7
.
 
~
~
*
*
N
e
w
 
i
n
o
d
e
 
c
h
u
n
k
s
.
*
*
~
~
 
 
D
o
n
e
,
 
o
n
 
a
 
g
r
o
u
p
 
w
i
t
h
 
a
n
 
e
m
p
t
y
 
i
n
o
d
e
 
t
r
e
e
 
*
a
n
d
*
 
o
n


 
 
 
o
n
e
 
w
h
o
s
e
 
i
n
o
d
e
 
t
r
e
e
 
h
a
s
 
t
o
 
s
p
l
i
t
 
t
o
 
m
a
k
e
 
r
o
o
m
,
 
w
i
t
h
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
a
c
c
e
p
t
i
n
g


 
 
 
b
o
t
h
.
 
 
W
h
a
t
 
i
s
 
s
t
i
l
l
 
n
o
t
 
e
s
t
a
b
l
i
s
h
e
d
 
i
s
 
t
h
e
 
f
r
e
e
 
s
l
o
t
s
'
 
l
a
y
o
u
t
 
i
n
 
a
n
 
i
m
a
g
e
 
X
F
S


 
 
 
i
t
s
e
l
f
 
b
u
i
l
t
:
 
t
h
e
 
l
a
y
o
u
t
 
h
e
r
e
 
i
s
 
m
e
a
s
u
r
e
d
 
a
g
a
i
n
s
t
 
r
e
p
a
i
r
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
o
n
l
y


 
 
 
a
u
t
h
o
r
i
t
y
 
a
v
a
i
l
a
b
l
e
 
h
e
r
e
,
 
a
n
d
 
n
o
 
c
h
u
n
k
 
i
n
 
a
n
y
 
i
m
a
g
e
 
i
n
 
t
h
i
s
 
r
e
p
o
s
i
t
o
r
y
 
w
a
s


 
 
 
c
r
e
a
t
e
d
 
b
y
 
a
n
 
o
p
e
r
a
t
i
o
n
 
a
n
y
o
n
e
 
h
e
r
e
 
c
a
n
 
w
a
t
c
h
.


8
.
 
*
*
F
i
l
e
 
e
x
t
e
n
s
i
o
n
 
a
n
d
 
h
o
l
e
s
*
*
 
—
 
b
o
t
h
 
a
l
r
e
a
d
y
 
w
o
r
k
 
f
o
r
 
t
h
e
 
c
o
n
t
i
g
u
o
u
s
 
c
a
s
e
;


 
 
 
w
h
a
t
 
r
e
m
a
i
n
s
 
i
s
 
m
a
k
i
n
g
 
t
h
e
m
 
s
u
r
v
i
v
e
 
t
h
e
 
a
l
l
o
c
a
t
o
r
 
w
o
r
k
 
a
b
o
v
e
.


9
.
 
*
*
B
M
B
T
 
g
r
o
w
t
h
 
a
n
d
 
d
i
r
e
c
t
o
r
i
e
s
*
*
,
 
i
n
 
t
h
a
t
 
o
r
d
e
r
.
 
 
T
r
u
n
c
a
t
e
 
i
s
 
d
o
n
e
:
 
a
 
f
i
l
e
 
m
a
d
e


 
 
 
s
h
o
r
t
e
r
 
g
i
v
e
s
 
i
t
s
 
b
l
o
c
k
s
 
b
a
c
k
,
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
a
c
c
e
p
t
s
 
t
h
e
 
i
m
a
g
e
,
 
w
h
i
c
h
 
i
s


 
 
 
t
h
e
 
f
i
r
s
t
 
o
p
e
r
a
t
i
o
n
 
h
e
r
e
 
j
u
d
g
e
d
 
e
n
d
 
t
o
 
e
n
d
 
o
n
 
a
 
f
i
l
e
 
t
h
e
 
r
e
s
t
 
o
f
 
t
h
e
 
f
i
l
e


 
 
 
s
y
s
t
e
m
 
c
a
n
 
s
t
i
l
l
 
f
i
n
d
.




 
 
 
T
h
e
 
e
x
t
e
n
t
 
b
o
u
n
d
a
r
y
 
i
n
 
f
r
o
n
t
 
o
f
 
t
h
e
 
f
i
r
s
t
 
i
s
 
p
i
n
n
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
c
r
o
s
s
e
d
:
 
a
 
f
i
l
e


 
 
 
n
e
e
d
i
n
g
 
a
 
t
e
n
t
h
 
e
x
t
e
n
t
 
i
s
 
r
e
f
u
s
e
d
 
w
i
t
h
 
`
E
N
O
S
Y
S
`
,
 
a
n
d
 
t
h
e
 
l
a
y
o
u
t
 
o
f
 
a
 
f
r
e
s
h


 
 
 
c
h
u
n
k
'
s
 
s
l
o
t
s
 
i
s
 
r
e
p
a
i
r
-
v
a
l
i
d
a
t
e
d
 
b
u
t
 
n
o
t
 
c
o
n
f
i
r
m
e
d
 
a
g
a
i
n
s
t
 
a
 
c
h
u
n
k
 
X
F
S
 
m
a
d
e
.


 
 
 
B
o
t
h
 
a
r
e
 
r
o
w
s
 
i
n
 
t
h
e


 
 
 
[
a
u
d
i
t
]
(
#
h
o
w
-
s
t
r
o
n
g
-
e
a
c
h
-
m
e
a
s
u
r
e
m
e
n
t
-
i
s
-
a
n
d
-
h
o
w
-
t
o
-
r
e
a
d
-
o
n
e
)
.




 
 
 
T
w
o
 
t
h
i
n
g
s
 
i
t
 
f
o
u
n
d
,
 
b
o
t
h
 
o
f
 
t
h
e
m
 
t
h
i
n
g
s
 
a
 
s
h
o
r
t
e
r
 
f
i
l
e
 
g
e
t
s
 
w
r
o
n
g
 
q
u
i
e
t
l
y
:




 
 
 
*
 
*
*
A
 
s
t
r
a
d
d
l
i
n
g
 
e
x
t
e
n
t
 
h
a
s
 
t
o
 
b
e
 
t
r
i
m
m
e
d
,
 
n
o
t
 
m
e
r
e
l
y
 
k
e
p
t
.
*
*
 
 
T
h
e
 
e
x
t
e
n
t
 
t
h
a
t


 
 
 
 
 
c
o
n
t
a
i
n
s
 
t
h
e
 
n
e
w
 
e
n
d
 
l
o
s
e
s
 
i
t
s
 
t
o
p
;
 
t
a
k
i
n
g
 
"
w
h
e
r
e
 
t
h
e
 
f
i
l
e
 
k
e
e
p
s
 
t
o
"
 
a
s


 
 
 
 
 
`
m
i
n
(
e
x
t
e
n
t
 
s
t
a
r
t
,
 
n
e
w
 
e
n
d
)
`
 
m
a
k
e
s
 
i
t
 
t
h
e
 
e
x
t
e
n
t
'
s
 
*
s
t
a
r
t
*
,
 
s
o
 
n
o
t
h
i
n
g
 
i
s


 
 
 
 
 
t
r
i
m
m
e
d
 
a
n
d
 
t
h
e
 
w
h
o
l
e
 
e
x
t
e
n
t
 
g
o
e
s
 
b
a
c
k
 
t
o
 
t
h
e
 
g
r
o
u
p
 
—
 
i
n
c
l
u
d
i
n
g
 
t
h
e
 
b
l
o
c
k
 
t
h
e


 
 
 
 
 
f
i
l
e
 
s
t
i
l
l
 
r
e
a
d
s
 
f
r
o
m
.
 
 
T
h
e
 
f
i
l
e
 
t
h
e
n
 
n
a
m
e
s
 
a
 
b
l
o
c
k
 
s
o
m
e
t
h
i
n
g
 
e
l
s
e
 
h
a
s
 
b
e
e
n


 
 
 
 
 
g
i
v
e
n
,
 
a
n
d
 
i
t
 
l
o
o
k
s
 
f
i
n
e
:
 
a
n
 
e
x
t
e
n
t
 
l
o
n
g
e
r
 
t
h
a
n
 
t
h
e
 
f
i
l
e
'
s
 
s
i
z
e
 
i
s
 
l
e
g
a
l
,
 
s
o


 
 
 
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
d
o
e
s
 
n
o
t
 
o
b
j
e
c
t
 
e
i
t
h
e
r
.




 
 
 
*
 
*
*
T
h
e
 
t
e
s
t
 
h
a
s
 
t
o
 
t
r
u
n
c
a
t
e
 
s
t
r
i
c
t
l
y
 
i
n
s
i
d
e
 
a
 
r
u
n
 
t
o
 
c
a
t
c
h
 
i
t
.
*
*
 
 
C
u
t
t
i
n
g
 
b
a
c
k


 
 
 
 
 
t
o
 
t
h
e
 
f
i
l
e
'
s
 
o
r
i
g
i
n
a
l
 
l
e
n
g
t
h
 
l
a
n
d
s
 
t
h
e
 
n
e
w
 
e
n
d
 
e
x
a
c
t
l
y
 
o
n
 
a
n
 
e
x
t
e
n
t


 
 
 
 
 
b
o
u
n
d
a
r
y
,
 
w
h
e
r
e
 
n
o
t
h
i
n
g
 
s
t
r
a
d
d
l
e
s
 
a
n
d
 
t
h
e
 
b
u
g
 
a
b
o
v
e
 
i
s
 
i
n
v
i
s
i
b
l
e
.
 
 
T
h
e
 
f
i
r
s
t


 
 
 
 
 
v
e
r
s
i
o
n
 
o
f
 
t
h
e
 
t
e
s
t
 
d
i
d
 
t
h
a
t
,
 
a
n
d
 
p
a
s
s
e
d
 
w
i
t
h
 
t
h
e
 
b
u
g
 
i
n
 
p
l
a
c
e
.
 
 
C
u
t
t
i
n
g
 
t
o
 
a


 
 
 
 
 
p
o
i
n
t
 
i
n
s
i
d
e
 
t
h
e
 
r
u
n
 
t
h
e
 
a
p
p
e
n
d
 
a
d
d
e
d
 
i
s
 
w
h
a
t
 
m
a
k
e
s
 
i
t
 
f
a
i
l
.




-
-
-




#
#
 
W
h
a
t
 
a
 
n
e
w
 
i
n
o
d
e
 
c
h
u
n
k
 
w
o
u
l
d
 
h
a
v
e
 
t
o
 
g
e
t
 
r
i
g
h
t




T
h
e
 
n
e
x
t
 
f
e
a
t
u
r
e
 
i
n
 
t
h
e
 
d
e
p
e
n
d
e
n
c
y
 
o
r
d
e
r
 
i
s
 
a
l
l
o
c
a
t
i
n
g
 
a
 
n
e
w
 
i
n
o
d
e
 
c
h
u
n
k
,
 
a
n
d
 
t
w
o


o
f
 
i
t
s
 
p
r
e
r
e
q
u
i
s
i
t
e
s
 
t
u
r
n
e
d
 
o
u
t
 
t
o
 
b
e
 
w
o
r
t
h
 
m
e
a
s
u
r
i
n
g
 
b
e
f
o
r
e
 
w
r
i
t
i
n
g
 
a
n
y
 
o
f
 
i
t
.


B
o
t
h
 
a
r
e
 
m
e
a
s
u
r
e
m
e
n
t
s
 
o
f
 
t
h
e
 
i
m
a
g
e
s
,
 
a
n
d
 
b
o
t
h
 
c
o
n
t
r
a
d
i
c
t
 
s
o
m
e
t
h
i
n
g
 
t
h
e
 
c
o
d
e
 
o
r
 
t
h
e


d
o
c
u
m
e
n
t
a
t
i
o
n
 
p
r
e
v
i
o
u
s
l
y
 
s
a
i
d
.




#
#
#
 
T
h
e
 
c
h
u
n
k
 
s
p
a
c
i
n
g
 
i
s
 
n
o
t
 
t
h
e
 
n
o
m
i
n
a
l
 
g
e
o
m
e
t
r
y
,
 
a
n
d
 
n
o
t
 
o
n
e
 
n
u
m
b
e
r




`
x
f
s
v
4
.
i
m
g
`
'
s
 
g
r
o
u
p
 
1
 
h
o
l
d
s
 
*
*
2
5
7
 
c
h
u
n
k
s
*
*
,
 
f
r
o
m
 
i
n
o
d
e
 
3
2
 
t
o
 
i
n
o
d
e
 
3
5
0
0
8
,
 
w
i
t
h


s
p
a
c
i
n
g
s
 
o
f
 
*
*
1
2
8
,
 
1
6
0
 
a
n
d
 
1
9
2
*
*
 
i
n
o
d
e
s
.
 
 
A
 
n
o
m
i
n
a
l
 
c
h
u
n
k
 
i
s
 
6
4
 
i
n
o
d
e
s
,
 
s
o
 
n
o
 
t
w
o


c
h
u
n
k
s
 
a
r
e
 
a
d
j
a
c
e
n
t
 
a
n
d
 
n
o
 
s
p
a
c
i
n
g
 
i
s
 
t
h
e
 
n
o
m
i
n
a
l
 
o
n
e
.
 
 
1
2
8
 
i
s
 
t
w
o
 
c
h
u
n
k
s
 
b
a
c
k
 
t
o


b
a
c
k
;
 
1
6
0
 
a
n
d
 
1
9
2
 
a
r
e
 
g
a
p
s
 
o
f
 
o
n
e
 
a
n
d
 
t
w
o
 
c
h
u
n
k
s
'
 
w
o
r
t
h
.




T
h
e
 
d
o
c
u
m
e
n
t
a
t
i
o
n
 
a
l
r
e
a
d
y
 
s
a
i
d
 
t
h
e
s
e
 
i
m
a
g
e
s
 
s
p
a
c
e
 
t
h
e
i
r
 
r
e
c
o
r
d
s
 
1
6
0
 
a
p
a
r
t
 
r
a
t
h
e
r


t
h
a
n
 
6
4
,
 
w
h
i
c
h
 
w
a
s
 
t
h
e
 
t
h
i
n
g
 
t
h
a
t
 
m
a
d
e
 
t
h
e
 
g
r
o
u
p
'
s
 
t
r
e
e
 
a
u
t
h
o
r
i
t
a
t
i
v
e
 
o
v
e
r


a
r
i
t
h
m
e
t
i
c
 
o
n
 
t
h
e
 
i
n
o
d
e
 
n
u
m
b
e
r
.
 
 
T
h
e
 
m
e
a
s
u
r
e
m
e
n
t
 
i
s
 
s
h
a
r
p
e
r
 
t
h
a
n
 
t
h
a
t
:
 
t
h
e
 
s
p
a
c
i
n
g


v
a
r
i
e
s
 
*
w
i
t
h
i
n
*
 
a
 
g
r
o
u
p
,
 
s
o
 
t
h
e
r
e
 
i
s
 
n
o
 
c
o
n
s
t
a
n
t
 
t
o
 
c
o
r
r
e
c
t
 
b
y
 
e
i
t
h
e
r
.
 
 
A
 
n
e
w


c
h
u
n
k
 
t
h
e
r
e
f
o
r
e
 
c
a
n
n
o
t
 
b
e
 
p
l
a
c
e
d
 
b
y
 
a
n
y
 
f
o
r
m
u
l
a
 
a
t
 
a
l
l
 
—
 
i
t
 
h
a
s
 
t
o
 
b
e
 
s
e
a
r
c
h
e
d


f
o
r
,
 
a
n
d
 
w
h
a
t
 
i
t
 
h
a
s
 
t
o
 
b
e
 
s
e
a
r
c
h
e
d
 
a
g
a
i
n
s
t
 
i
s
 
t
h
e
 
g
r
o
u
p
'
s
 
o
w
n
 
f
r
e
e
 
s
p
a
c
e
 
a
n
d
 
i
t
s


o
w
n
 
m
e
t
a
d
a
t
a
.




#
#
#
 
A
 
f
r
e
e
 
i
n
o
d
e
'
s
 
s
l
o
t
 
i
s
 
*
*
n
o
t
*
*
 
a
l
l
 
z
e
r
o
e
s
,
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
`
 
s
a
y
s
 
e
x
a
c
t
l
y
 
w
h
a
t




T
h
e
 
f
i
r
s
t
 
r
e
a
d
i
n
g
 
o
f
 
t
h
i
s
 
w
a
s
 
w
r
o
n
g
 
a
n
d
 
i
s
 
w
o
r
t
h
 
r
e
c
o
r
d
i
n
g
 
b
e
c
a
u
s
e
 
i
t
 
n
e
a
r
l
y


b
e
c
a
m
e
 
t
h
e
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
.
 
 
`
x
f
s
v
4
.
i
m
g
`
'
s
 
g
r
o
u
p
 
1
 
h
a
s
 
5
2
 
f
r
e
e
 
i
n
o
d
e
s
 
a
n
d
 
t
h
e
i
r


s
l
o
t
s
 
*
l
o
o
k
e
d
*
 
l
i
k
e
 
r
u
n
s
 
o
f
 
z
e
r
o
e
s
 
—
 
a
 
h
a
n
d
 
p
a
r
s
e
r
 
r
e
a
d
 
t
h
e
m
 
o
u
t
 
o
f
 
b
o
u
n
d
s
,
 
a
n
d
 
t
h
e


m
i
s
t
a
k
e
 
s
u
r
v
i
v
e
d
 
a
 
g
o
o
d
 
w
h
i
l
e
 
—
 
s
o
 
a
 
c
h
u
n
k
'
s
 
6
4
 
s
l
o
t
s
 
w
e
r
e
 
d
u
l
y
 
w
r
i
t
t
e
n
 
t
h
a
t
 
w
a
y
.


`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
r
e
f
u
s
e
d
:




`
`
`
t
e
x
t


b
a
d
 
m
a
g
i
c
 
n
u
m
b
e
r
 
0
x
0
 
o
n
 
i
n
o
d
e
 
9
5
,
 
w
o
u
l
d
 
r
e
s
e
t
 
m
a
g
i
c
 
n
u
m
b
e
r


b
a
d
 
v
e
r
s
i
o
n
 
n
u
m
b
e
r
 
0
x
0
 
o
n
 
i
n
o
d
e
 
9
5
,
 
w
o
u
l
d
 
r
e
s
e
t
 
v
e
r
s
i
o
n
 
n
u
m
b
e
r


b
a
d
 
n
e
x
t
_
u
n
l
i
n
k
e
d
 
0
x
0
 
o
n
 
i
n
o
d
e
 
9
5
,
 
w
o
u
l
d
 
r
e
s
e
t
 
n
e
x
t
_
u
n
l
i
n
k
e
d


f
r
e
e
 
i
n
o
d
e
 
9
5
 
c
o
n
t
a
i
n
s
 
e
r
r
o
r
s
,
 
w
o
u
l
d
 
c
o
r
r
e
c
t


`
`
`




S
o
 
t
h
e
 
l
a
y
o
u
t
 
w
a
s
 
*
*
m
e
a
s
u
r
e
d
*
*
,
 
f
i
e
l
d
 
b
y
 
f
i
e
l
d
,
 
b
y
 
a
s
k
i
n
g
 
t
h
e
 
t
o
o
l
:




|
 
f
i
e
l
d
 
|
 
o
f
f
s
e
t
 
|
 
v
a
l
u
e
 
|
 
h
o
w
 
|


|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
|


|
 
i
n
o
d
e
 
m
a
g
i
c
 
|
 
0
 
|
 
`
0
x
4
9
4
e
`
 
|
 
n
a
m
e
d
 
b
y
 
r
e
p
a
i
r
 
|


|
 
v
e
r
s
i
o
n
 
|
 
4
 
|
 
2
 
f
o
r
 
a
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
,
 
3
 
f
o
r
 
a
 
l
a
r
g
e
r
 
o
n
e
 
|
 
`
x
f
s
_
d
b
`
 
o
n
 
t
h
e
 
i
m
a
g
e
'
s
 
o
w
n
 
i
n
o
d
e
s
 
|


|
 
`
n
e
x
t
_
u
n
l
i
n
k
e
d
`
 
|
 
*
*
9
6
*
*
 
|
 
`
0
x
f
f
f
f
f
f
f
f
`
 
|
 
r
e
p
a
i
r
 
n
a
m
e
s
 
t
h
e
 
f
i
e
l
d
;
 
i
t
s
 
o
f
f
s
e
t
 
a
n
d
 
v
a
l
u
e
 
f
o
u
n
d
 
b
y
 
w
r
i
t
i
n
g
 
a
 
r
e
c
o
g
n
i
s
a
b
l
e
 
v
a
l
u
e
 
a
t
 
e
a
c
h
 
o
f
f
s
e
t
 
a
n
d
 
a
s
k
i
n
g
 
w
h
i
c
h
 
o
n
e
 
r
e
p
a
i
r
 
r
e
a
d
 
|




T
h
e
 
o
f
f
s
e
t
 
i
s
 
t
h
e
 
o
n
e
 
t
h
a
t
 
w
o
u
l
d
 
n
o
t
 
h
a
v
e
 
b
e
e
n
 
g
u
e
s
s
e
d
:
 
`
n
e
x
t
_
u
n
l
i
n
k
e
d
`
 
s
i
t
s


*
*
i
m
m
e
d
i
a
t
e
l
y
 
a
f
t
e
r
 
t
h
e
 
c
o
r
e
,
 
a
t
 
9
6
*
*
,
 
n
o
t
 
a
t
 
t
h
e
 
e
n
d
 
o
f
 
t
h
e
 
i
n
o
d
e
,
 
w
h
i
c
h
 
i
s
 
w
h
e
r
e


t
h
e
 
o
b
v
i
o
u
s
 
g
u
e
s
s
 
p
u
t
s
 
i
t
 
a
n
d
 
w
h
e
r
e
 
t
h
e
 
f
i
r
s
t
 
v
e
r
s
i
o
n
 
w
r
o
t
e
 
`
0
x
f
f
f
f
f
f
f
f
`
 
—
 
a
n
d


w
h
i
c
h
 
i
s
 
w
h
y
 
t
h
e
 
f
i
r
s
t
 
a
t
t
e
m
p
t
 
s
t
i
l
l
 
g
o
t
 
a
 
`
b
a
d
 
n
e
x
t
_
u
n
l
i
n
k
e
d
 
0
x
0
`
.
 
 
T
h
e
 
v
a
l
u
e
 
i
s


t
h
e
 
l
i
s
t
'
s
 
e
n
d
 
m
a
r
k
e
r
,
 
a
n
d
 
n
o
t
h
i
n
g
 
e
l
s
e
 
i
s
 
a
c
c
e
p
t
e
d
:
 
z
e
r
o
,
 
o
n
e
,
 
t
h
e
 
i
n
o
d
e
'
s
 
o
w
n


n
u
m
b
e
r
 
a
n
d
 
`
0
x
f
f
f
f
f
f
f
e
`
 
w
e
r
e
 
e
a
c
h
 
t
r
i
e
d
 
a
n
d
 
e
a
c
h
 
r
e
f
u
s
e
d
.




W
h
a
t
 
i
s
 
s
t
i
l
l
 
*
*
u
n
m
e
a
s
u
r
e
d
*
*
 
i
s
 
w
h
a
t
 
X
F
S
 
w
r
i
t
e
s
 
w
h
e
n
 
i
t
 
*
c
r
e
a
t
e
s
*
 
a
 
c
h
u
n
k
:
 
n
o
 
c
h
u
n
k


i
n
 
a
n
y
 
i
m
a
g
e
 
h
e
r
e
 
w
a
s
 
c
r
e
a
t
e
d
 
b
y
 
a
n
 
o
p
e
r
a
t
i
o
n
 
t
h
i
s
 
s
u
i
t
e
 
c
a
n
 
w
a
t
c
h
.
 
 
W
h
a
t
 
i
s
 
a
b
o
v
e


i
s
 
w
h
a
t
 
a
 
c
h
u
n
k
 
m
u
s
t
 
l
o
o
k
 
l
i
k
e
 
f
o
r
 
r
e
p
a
i
r
 
t
o
 
a
c
c
e
p
t
 
i
t
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
m
o
s
t
 
t
h
a
t
 
c
a
n


b
e
 
e
s
t
a
b
l
i
s
h
e
d
 
h
e
r
e
 
a
n
d
 
i
s
 
e
n
o
u
g
h
 
t
o
 
w
r
i
t
e
 
o
n
e
.




#
#
#
 
T
h
e
 
r
e
c
o
r
d
'
s
 
`
s
t
a
r
t
i
n
o
`
 
i
s
 
c
o
u
n
t
e
d
 
f
r
o
m
 
t
h
e
 
s
t
a
r
t
 
o
f
 
t
h
e
 
g
r
o
u
p




A
n
o
t
h
e
r
 
t
h
i
n
g
 
t
h
a
t
 
i
s
 
i
n
v
i
s
i
b
l
e
 
u
n
t
i
l
 
s
o
m
e
t
h
i
n
g
 
r
e
a
d
s
 
t
h
e
 
r
e
c
o
r
d
 
b
a
c
k
 
a
n
d
 
t
u
r
n
s
 
i
t


i
n
t
o
 
a
n
 
i
n
o
d
e
 
n
u
m
b
e
r
.
 
 
`
a
l
l
o
c
a
t
e
_
n
e
w
_
c
h
u
n
k
`
 
f
i
r
s
t
 
w
r
o
t
e
 
a
n
 
*
a
b
s
o
l
u
t
e
*
 
i
n
o
d
e
 
n
u
m
b
e
r
,


a
n
d
 
a
 
s
w
e
e
p
 
o
v
e
r
 
t
h
e
 
f
i
e
l
d
'
s
 
w
h
o
l
e
 
p
l
a
u
s
i
b
l
e
 
r
a
n
g
e
 
s
h
o
w
e
d
 
w
h
a
t
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`


a
c
c
e
p
t
s
:




`
`
`
t
e
x
t


s
t
a
r
t
i
n
o
 
<
 
 
3
0
7
2
0
0
:
 
 
"
i
n
o
d
e
 
c
h
u
n
k
 
c
l
a
i
m
s
 
u
s
e
d
 
b
l
o
c
k
"
 
-
-
 
t
h
e
 
n
u
m
b
e
r
 
i
s
 
i
n
 
r
a
n
g
e


s
t
a
r
t
i
n
o
 
>
=
 
3
0
7
2
0
0
:
 
 
"
b
a
d
 
s
t
a
r
t
i
n
g
 
i
n
o
d
e
"
 
 
 
 
 
 
 
 
 
 
 
 
-
-
 
t
h
e
 
n
u
m
b
e
r
 
i
s
 
n
o
t


`
`
`




3
0
7
2
0
0
 
i
s
 
`
1
5
3
6
0
0
 
×
 
2
`
,
 
t
h
e
 
n
u
m
b
e
r
 
o
f
 
i
n
o
d
e
s
 
*
*
i
n
 
t
h
e
 
g
r
o
u
p
*
*
.
 
 
S
o
 
t
h
e
 
f
i
e
l
d
 
i
s


b
o
u
n
d
e
d
 
b
y
 
t
h
e
 
g
r
o
u
p
'
s
 
o
w
n
 
i
n
o
d
e
 
c
o
u
n
t
,
 
n
o
t
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
'
s
:
 
i
t
 
i
s
 
a


g
r
o
u
p
-
r
e
l
a
t
i
v
e
 
n
u
m
b
e
r
,
 
a
n
d
 
`
S
b
:
:
m
a
k
e
_
i
n
o
`
 
a
d
d
s
 
t
h
e
 
b
a
s
e
.
 
 
T
h
e
 
r
e
p
o
s
i
t
o
r
y
'
s
 
e
x
i
s
t
i
n
g


c
o
d
e
 
a
l
r
e
a
d
y
 
d
i
d
 
t
h
i
s
 
c
o
r
r
e
c
t
l
y
 
—
 
`
x
f
s
v
4
.
i
m
g
`
'
s
 
g
r
o
u
p
 
1
 
h
o
l
d
s
 
a
 
c
h
u
n
k
 
r
e
c
o
r
d
e
d
 
a
t


3
5
0
0
8
 
a
n
d
 
i
t
s
 
h
i
n
t
 
s
a
y
s
 
3
5
0
0
8
,
 
b
o
t
h
 
f
a
r
 
b
e
l
o
w
 
t
h
a
t
 
g
r
o
u
p
'
s
 
a
b
s
o
l
u
t
e
 
f
i
r
s
t
 
i
n
o
d
e
 
o
f


6
5
5
3
6
 
—
 
s
o
 
t
h
e
 
m
i
s
t
a
k
e
 
w
a
s
 
m
i
n
e
,
 
a
n
d
 
t
h
e
 
l
e
s
s
o
n
 
i
t
 
p
r
o
d
u
c
e
d
 
i
s
 
r
e
c
o
r
d
e
d
:
 
I
 
f
i
r
s
t


c
o
n
c
l
u
d
e
d
 
t
h
a
t
 
X
F
S
 
a
n
d
 
`
S
b
:
:
l
o
c
a
t
e
_
i
n
o
`
 
d
i
s
a
g
r
e
e
d
 
a
b
o
u
t
 
w
h
e
r
e
 
a
 
g
r
o
u
p
'
s
 
i
n
o
d
e
s


s
t
a
r
t
.
 
 
T
h
e
y
 
d
o
 
n
o
t
.
 
 
T
h
e
 
d
i
s
a
g
r
e
e
m
e
n
t
 
w
a
s
 
t
h
a
t
 
a
 
g
r
o
u
p
-
r
e
l
a
t
i
v
e
 
f
i
e
l
d
 
h
a
d
 
b
e
e
n


g
i
v
e
n
 
a
n
 
a
b
s
o
l
u
t
e
 
n
u
m
b
e
r
.




#
#
#
 
A
n
 
a
l
l
o
c
a
t
e
d
 
i
n
o
d
e
 
i
s
 
*
d
i
s
c
o
n
n
e
c
t
e
d
*
,
 
a
n
d
 
n
o
 
t
e
s
t
 
c
a
n
 
a
s
k
 
r
e
p
a
i
r
 
t
o
 
a
c
c
e
p
t
 
i
t




A
l
l
o
c
a
t
i
n
g
 
a
n
 
i
n
o
d
e
 
w
i
t
h
o
u
t
 
a
 
`
c
r
e
a
t
e
`
 
t
o
 
l
i
n
k
 
i
t
 
i
n
t
o
 
a
 
d
i
r
e
c
t
o
r
y
 
l
e
a
v
e
s
 
a
n


i
n
o
d
e
 
n
o
 
d
i
r
e
c
t
o
r
y
 
c
a
n
 
r
e
a
c
h
,
 
a
n
d
 
X
F
S
'
s
 
w
o
r
d
 
f
o
r
 
t
h
a
t
 
i
s
 
*
*
d
i
s
c
o
n
n
e
c
t
e
d
*
*
:




`
`
`
t
e
x
t


d
i
s
c
o
n
n
e
c
t
e
d
 
i
n
o
d
e
 
5
2
4
3
5
2
,
 
w
o
u
l
d
 
m
o
v
e
 
t
o
 
l
o
s
t
+
f
o
u
n
d


`
`
`




`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
'
s
 
r
e
m
e
d
y
 
i
s
 
t
o
 
m
o
v
e
 
i
t
 
t
o
 
l
o
s
t
+
f
o
u
n
d
.
 
 
S
o
 
a
n
 
i
m
a
g
e
 
o
n
 
w
h
i
c
h
 
t
h
i
s


p
r
o
g
r
a
m
 
h
a
s
 
a
l
l
o
c
a
t
e
d
 
a
n
 
i
n
o
d
e
 
c
a
n
n
o
t
 
b
e
 
o
n
e
 
r
e
p
a
i
r
 
a
c
c
e
p
t
s
,
 
a
n
d
 
t
h
e
 
o
n
l
y
 
h
o
n
e
s
t


a
r
r
a
n
g
e
m
e
n
t
 
i
s
 
t
o
 
a
s
k
 
r
e
p
a
i
r
 
w
h
i
l
e
 
t
h
e
 
c
h
u
n
k
 
e
x
i
s
t
s
 
a
n
d
 
e
v
e
r
y
 
o
n
e
 
o
f
 
i
t
s
 
i
n
o
d
e
s
 
i
s


s
t
i
l
l
 
f
r
e
e
 
—
 
w
h
i
c
h
 
i
t
 
a
c
c
e
p
t
s
 
—
 
a
n
d
 
t
h
e
n
 
t
o
 
a
s
s
e
r
t
 
t
h
a
t
 
t
h
e
 
i
n
o
d
e
'
s
 
a
l
l
o
c
a
t
i
o
n
 
i
s


t
h
e
 
*
o
n
l
y
*
 
t
h
i
n
g
 
r
e
p
a
i
r
 
o
b
j
e
c
t
s
 
t
o
 
a
f
t
e
r
w
a
r
d
s
.




#
#
#
 
A
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
 
h
o
l
d
s
 
n
i
n
e
 
e
x
t
e
n
t
s
,
 
a
n
d
 
t
h
e
 
t
e
n
t
h
 
i
s
 
r
e
f
u
s
e
d




W
o
r
t
h
 
w
r
i
t
i
n
g
 
d
o
w
n
 
b
e
c
a
u
s
e
 
t
h
e
 
n
u
m
b
e
r
 
i
s
 
s
m
a
l
l
,
 
i
t
 
i
s
 
n
o
t
 
w
h
a
t
 
i
t
 
l
o
o
k
s
 
l
i
k
e
,
 
a
n
d


t
h
e
 
w
r
o
n
g
 
a
n
s
w
e
r
 
i
s
 
s
i
l
e
n
t
.
 
 
A
 
2
5
6
-
b
y
t
e
 
i
n
o
d
e
'
s
 
l
o
c
a
l
 
a
r
e
a
 
s
t
a
r
t
s
 
a
t
 
b
y
t
e
 
1
0
0
 
a
n
d


a
n
 
e
x
t
e
n
t
 
r
e
c
o
r
d
 
i
s
 
1
6
 
b
y
t
e
s
,
 
s
o
 
`
(
2
5
6
 
−
 
1
0
0
)
 
/
 
1
6
`
 
i
s
 
*
*
n
i
n
e
*
*
 
o
f
 
t
h
e
m
.
 
 
M
e
a
s
u
r
e
d


e
n
d
 
t
o
 
e
n
d
:
 
n
i
n
e
 
s
p
a
r
s
e
 
w
r
i
t
e
s
 
s
u
c
c
e
e
d
 
a
n
d
 
t
h
e
 
t
e
n
t
h
 
i
s
 
r
e
f
u
s
e
d
.




T
h
e
 
r
e
f
u
s
a
l
 
i
s
 
t
h
e
 
p
o
i
n
t
 
a
s
 
m
u
c
h
 
a
s
 
t
h
e
 
l
i
m
i
t
.
 
 
A
 
f
i
l
e
 
w
h
o
s
e
 
e
x
t
e
n
t
s
 
d
o
 
n
o
t
 
f
i
t


h
a
s
 
t
o
 
b
e
 
e
i
t
h
e
r
 
m
o
v
e
d
 
i
n
t
o
 
a
 
b
-
t
r
e
e
 
o
r
 
r
e
f
u
s
e
d
,
 
b
e
c
a
u
s
e
 
t
h
e
 
a
l
t
e
r
n
a
t
i
v
e
s
 
—


w
r
i
t
i
n
g
 
a
 
t
e
n
t
h
 
r
e
c
o
r
d
 
o
v
e
r
 
t
h
e
 
a
t
t
r
i
b
u
t
e
 
f
o
r
k
,
 
o
r
 
o
v
e
r
 
t
h
e
 
i
n
o
d
e
'
s
 
o
w
n
 
t
a
i
l
 
—


p
r
o
d
u
c
e
 
a
 
f
i
l
e
 
t
h
a
t
 
r
e
a
d
s
 
b
a
c
k
 
a
s
 
s
o
m
e
t
h
i
n
g
 
e
l
s
e
.
 
 
A
n
d
 
t
h
e
 
r
e
f
u
s
a
l
 
i
s
 
`
E
N
O
S
Y
S
`


r
a
t
h
e
r
 
t
h
a
n
 
`
E
N
O
S
P
C
`
,
 
b
e
c
a
u
s
e
 
t
h
i
s
 
i
s
 
n
o
t
 
t
h
e
 
d
e
v
i
c
e
 
r
u
n
n
i
n
g
 
o
u
t
 
a
n
d
 
a
 
c
a
l
l
e
r
 
t
o
l
d


"
n
o
 
s
p
a
c
e
"
 
w
o
u
l
d
 
g
o
 
l
o
o
k
i
n
g
 
f
o
r
 
s
p
a
c
e
 
t
h
a
t
 
i
s
 
n
o
t
 
t
h
e
 
p
r
o
b
l
e
m
.




W
h
a
t
 
i
s
 
*
*
n
o
t
*
*
 
e
s
t
a
b
l
i
s
h
e
d
 
i
s
 
w
h
a
t
 
h
a
p
p
e
n
s
 
o
n
 
t
h
e
 
o
t
h
e
r
 
s
i
d
e
.
 
 
T
h
e
 
f
o
r
m
a
t
'
s
 
a
n
s
w
e
r


i
s
 
a
 
b
-
t
r
e
e
 
r
o
o
t
e
d
 
i
n
 
t
h
e
 
i
n
o
d
e
,
 
a
n
d
 
t
h
a
t
 
n
e
e
d
s
 
a
 
*
r
e
a
d
e
r
*
 
a
s
 
w
e
l
l
 
a
s
 
a
 
w
r
i
t
e
r
:


t
h
i
s
 
c
o
d
e
 
h
a
s
 
n
o
 
b
-
m
a
p
 
b
l
o
c
k
 
d
e
c
o
d
e
r
 
a
t
 
a
l
l
,
 
s
o
 
e
v
e
n
 
a
 
s
p
i
l
l
 
t
h
a
t
 
w
r
o
t
e
 
a


w
e
l
l
-
f
o
r
m
e
d
 
l
e
a
f
 
c
o
u
l
d
 
n
o
t
 
r
e
a
d
 
t
h
e
 
f
i
l
e
 
b
a
c
k
.
 
 
N
e
i
t
h
e
r
 
i
s
 
w
r
i
t
t
e
n
,
 
d
e
l
i
b
e
r
a
t
e
l
y
 
—


a
 
s
e
c
o
n
d
 
b
-
t
r
e
e
 
t
h
a
t
 
c
a
n
n
o
t
 
b
e
 
r
e
a
d
 
i
s
 
t
h
e
 
k
i
n
d
 
o
f
 
t
h
i
n
g
 
t
h
a
t
 
s
h
o
u
l
d
 
n
o
t
 
l
a
n
d
.




#
#
#
 
A
 
r
e
a
l
 
b
-
t
r
e
e
 
d
a
t
a
 
f
o
r
k
 
e
x
i
s
t
s
 
i
n
 
`
x
f
s
v
4
.
i
m
g
`
,
 
a
n
d
 
n
e
i
t
h
e
r
 
t
h
i
s
 
c
o
d
e
 
n
o
r
 
`
x
f
s
_
d
b
`
 
c
a
n
 
d
e
c
o
d
e
 
i
t
s
 
b
l
o
c
k
s




T
h
e
 
n
e
x
t
 
t
h
i
n
g
 
i
n
 
o
r
d
e
r
 
i
s
 
a
 
f
i
l
e
 
w
h
o
s
e
 
e
x
t
e
n
t
s
 
n
o
 
l
o
n
g
e
r
 
f
i
t
 
i
n
 
i
t
s
 
i
n
o
d
e
.
 
 
T
h
e


f
o
r
m
a
t
'
s
 
a
n
s
w
e
r
 
i
s
 
a
 
b
-
t
r
e
e
 
r
o
o
t
e
d
 
i
n
 
t
h
e
 
i
n
o
d
e
,
 
a
n
d
 
t
h
a
t
 
t
u
r
n
s
 
o
u
t
 
t
o
 
b
e


*
r
e
a
c
h
a
b
l
e
*
 
r
a
t
h
e
r
 
t
h
a
n
 
h
y
p
o
t
h
e
t
i
c
a
l
:
 
`
x
f
s
v
4
.
i
m
g
`
 
h
a
s
 
t
h
r
e
e
 
r
e
g
u
l
a
r
 
f
i
l
e
s
 
w
h
o
s
e


d
a
t
a
 
f
o
r
k
 
i
s
 
a
 
b
-
t
r
e
e
,
 
a
n
d
 
o
n
e
 
o
f
 
t
h
e
m
 
i
s
 
s
m
a
l
l
 
e
n
o
u
g
h
 
t
o
 
r
e
a
d
 
c
o
m
f
o
r
t
a
b
l
y
.




`
`
`
t
e
x
t


i
n
o
d
e
 
1
0
0
5
5
3
 
 
m
o
d
e
 
0
1
0
0
6
4
4
 
f
m
t
 
b
t
r
e
e
 
 
n
e
x
 
6
4
 
 
n
b
l
k
 
6
7
 
 
s
z
 
3
2
7
6
8


 
 
 
 
 
 
 
 
 
 
 
 
 
 
u
.
b
m
b
t
.
l
e
v
e
l
 
=
 
1
 
 
 
u
.
b
m
b
t
.
n
u
m
r
e
c
s
 
=
 
3


 
 
 
 
 
 
 
 
 
 
 
 
 
 
u
.
b
m
b
t
.
k
e
y
s
[
1
-
3
]
 
=
 
[
s
t
a
r
t
o
f
f
]
 
 
 
1
:
[
0
]
 
 
2
:
[
3
0
]
 
 
3
:
[
4
5
]


 
 
 
 
 
 
 
 
 
 
 
 
 
 
u
.
b
m
b
t
.
p
t
r
s
[
1
-
3
]
 
=
 
1
:
5
0
3
1
3
 
 
2
:
5
0
3
1
5
 
 
3
:
5
0
3
1
7


`
`
`




S
o
 
t
h
e
 
*
*
f
o
r
k
*
*
 
l
a
y
o
u
t
 
i
s
 
s
e
t
t
l
e
d
 
b
y
 
`
x
f
s
_
d
b
`
'
s
 
o
w
n
 
p
r
i
n
t
 
o
f
 
a
 
r
e
a
l
 
i
n
o
d
e
 
a
n
d


a
g
r
e
e
s
 
w
i
t
h
 
w
h
a
t
 
t
h
i
s
 
c
o
d
e
 
d
e
c
o
d
e
s
:
 
a
 
l
e
v
e
l
,
 
a
 
r
e
c
o
r
d
 
c
o
u
n
t
,
 
t
h
a
t
 
m
a
n
y
 
k
e
y
s
,
 
a
n
d


t
h
a
t
 
m
a
n
y
 
p
o
i
n
t
e
r
s
.
 
 
T
h
a
t
 
m
u
c
h
 
i
s
 
c
o
n
f
i
r
m
e
d
.




T
h
e
 
*
*
b
l
o
c
k
*
*
 
l
a
y
o
u
t
 
i
s
 
n
o
t
,
 
a
n
d
 
t
h
e
 
r
e
a
s
o
n
 
i
s
 
w
o
r
t
h
 
r
e
c
o
r
d
i
n
g
:
 
*
*
`
x
f
s
_
d
b
`
 
h
a
s
 
n
o


b
-
m
a
p
 
b
l
o
c
k
 
r
e
a
d
e
r
 
e
i
t
h
e
r
.
*
*
 
 
I
t
s
 
t
y
p
e
 
l
i
s
t
 
h
a
s
 
`
a
t
t
r
`
,
 
`
b
n
o
b
t
`
 
a
n
d
 
`
i
n
o
b
t
`
 
b
u
t


n
o
t
h
i
n
g
 
f
o
r
 
a
 
f
i
l
e
 
m
a
p
p
i
n
g
 
t
r
e
e
;
 
a
s
k
e
d
 
t
o
 
d
u
m
p
 
o
n
e
 
o
f
 
t
h
o
s
e
 
b
l
o
c
k
s
 
i
t
 
a
n
s
w
e
r
s


`
n
o
 
c
u
r
r
e
n
t
 
t
y
p
e
`
,
 
a
n
d
 
a
s
k
e
d
 
t
o
 
`
b
t
d
u
m
p
`
 
i
t
 
s
a
y
s
 
`
t
y
p
e
 
"
d
a
t
a
"
 
i
s
 
n
o
t
 
a
 
b
t
r
e
e
 
t
y
p
e


o
r
 
i
n
o
d
e
`
.
 
 
S
o
 
t
h
e
 
t
o
o
l
 
t
h
a
t
 
r
e
a
d
s
 
e
v
e
r
y
 
o
t
h
e
r
 
s
t
r
u
c
t
u
r
e
 
h
e
r
e
 
c
a
n
n
o
t
 
s
h
o
w
 
t
h
e


r
e
c
o
r
d
s
 
i
n
s
i
d
e
 
a
 
b
-
m
a
p
 
b
l
o
c
k
,
 
a
n
d
 
t
h
e
 
l
a
y
o
u
t
 
h
a
s
 
t
o
 
b
e
 
e
s
t
a
b
l
i
s
h
e
d
 
t
h
e
 
w
a
y


e
v
e
r
y
t
h
i
n
g
 
e
l
s
e
 
i
n
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
n
o
w
 
i
s
:
 
b
u
i
l
d
 
o
n
e
 
a
n
d
 
a
s
k
 
`
x
f
s
_
r
e
p
a
i
r
`
.




T
h
e
 
*
f
o
r
k
*
 
l
a
y
o
u
t
 
i
s
 
n
o
w
 
p
i
n
n
e
d
,
 
b
y
t
e
 
f
o
r
 
b
y
t
e
,
 
a
g
a
i
n
s
t
 
a
 
r
e
a
l
 
i
n
o
d
e
:




`
`
`
t
e
x
t


 
 
1
0
0
 
 
l
e
v
e
l
 
 
 
 
 
u
1
6


 
 
1
0
2
 
 
n
u
m
r
e
c
s
 
 
 
u
1
6


 
 
1
0
4
 
 
k
e
y
s
[
0
.
.
n
u
m
r
e
c
s
]
 
 
 
s
t
a
r
t
o
f
f
,
 
u
6
4
 
e
a
c
h


 
 
1
7
6
 
 
p
t
r
s
[
0
.
.
n
u
m
r
e
c
s
]
 
 
 
b
l
o
c
k
,
 
 
 
 
u
6
4
 
e
a
c
h
 
 
 
 
 
(
1
0
4
 
+
 
3
*
8
 
+
 
4
8
 
=
 
1
7
6
)


`
`
`




a
n
d
 
t
h
e
 
g
a
p
 
b
e
t
w
e
e
n
 
t
h
e
 
a
r
r
a
y
s
 
i
s
 
`
d
f
o
r
k
_
b
t
r
e
e
_
p
t
r
_
g
a
p
(
i
n
o
d
e
 
s
i
z
e
,
 
n
u
m
r
e
c
s
)
`
,
 
s
o


t
h
e
 
p
o
i
n
t
e
r
 
o
f
f
s
e
t
 
i
s
 
n
o
t
 
a
 
c
o
n
s
t
a
n
t
 
a
n
d
 
c
a
n
n
o
t
 
b
e
 
w
r
i
t
t
e
n
 
d
o
w
n
 
a
s
 
o
n
e
.
 
 
A
 
t
e
s
t
 
n
o
w


a
s
k
s
 
t
h
i
s
 
c
o
d
e
 
a
n
d
 
`
x
f
s
_
d
b
`
 
t
h
e
 
s
a
m
e
 
q
u
e
s
t
i
o
n
 
a
b
o
u
t
 
i
n
o
d
e
 
1
0
0
5
5
3
'
s
 
f
o
r
k
 
a
n
d


r
e
q
u
i
r
e
s
 
t
h
e
m
 
t
o
 
a
g
r
e
e
 
a
b
o
u
t
 
t
h
e
 
l
e
v
e
l
,
 
t
h
e
 
r
e
c
o
r
d
 
c
o
u
n
t
,
 
e
v
e
r
y
 
k
e
y
 
a
n
d
 
e
v
e
r
y


p
o
i
n
t
e
r
 
-
-
 
w
h
i
c
h
 
i
s
 
*
*
s
h
i
p
p
e
d
 
c
o
d
e
 
b
e
i
n
g
 
c
h
e
c
k
e
d
 
a
g
a
i
n
s
t
 
a
 
r
e
a
l
 
f
i
l
e
 
f
o
r
 
t
h
e
 
f
i
r
s
t


t
i
m
e
*
*
,
 
s
i
n
c
e
 
e
v
e
r
y
 
i
n
o
d
e
 
a
n
y
 
o
t
h
e
r
 
t
e
s
t
 
r
e
a
d
s
 
h
a
s
 
i
t
s
 
e
x
t
e
n
t
s
 
*
i
n
*
 
t
h
e
 
i
n
o
d
e
.




#
#
#
 
T
h
e
 
b
-
m
a
p
 
b
l
o
c
k
:
 
w
h
a
t
 
t
h
r
e
e
 
l
e
a
v
e
s
 
a
g
r
e
e
 
o
n
,
 
a
n
d
 
t
h
e
 
o
n
e
 
t
h
i
n
g
 
t
h
e
y
 
d
o
 
n
o
t




T
h
e
 
r
e
c
o
r
d
 
l
a
y
o
u
t
 
*
i
n
s
i
d
e
*
 
a
 
b
-
m
a
p
 
b
l
o
c
k
 
i
s
 
t
h
e
 
l
a
s
t
 
p
i
e
c
e
 
o
f
 
t
h
i
s
,
 
a
n
d
 
i
t
 
i
s
 
w
o
r
t
h


w
r
i
t
i
n
g
 
d
o
w
n
 
a
s
 
f
a
r
 
a
s
 
i
t
 
a
c
t
u
a
l
l
y
 
g
o
e
s
,
 
b
e
c
a
u
s
e
 
m
o
s
t
 
o
f
 
i
t
 
n
o
w
 
r
e
s
t
s
 
o
n
 
t
h
r
e
e


i
n
d
e
p
e
n
d
e
n
t
 
c
o
n
f
i
r
m
a
t
i
o
n
s
 
r
a
t
h
e
r
 
t
h
a
n
 
o
n
 
o
n
e
 
r
e
a
d
i
n
g
.




`
x
f
s
_
d
b
`
 
c
a
n
n
o
t
 
h
e
l
p
:
 
i
t
s
 
t
y
p
e
 
l
i
s
t
 
h
a
s
 
`
a
t
t
r
`
,
 
`
b
n
o
b
t
`
,
 
`
i
n
o
b
t
`
 
a
n
d
 
t
h
e
 
t
w
o


`
b
m
a
p
b
*
`
 
n
a
m
e
s
,
 
a
n
d
 
a
s
k
e
d
 
t
o
 
d
e
c
o
d
e
 
a
 
b
-
m
a
p
 
b
l
o
c
k
 
w
i
t
h
 
a
n
y
 
o
f
 
t
h
e
m
 
i
t
 
f
a
l
l
s
 
b
a
c
k
 
t
o


a
 
r
a
w
 
h
e
x
 
d
u
m
p
 
o
r
 
a
n
s
w
e
r
s
 
`
n
o
 
c
u
r
r
e
n
t
 
t
y
p
e
`
.
 
 
S
o
 
t
h
e
 
b
l
o
c
k
 
w
a
s
 
r
e
a
d
 
d
i
r
e
c
t
l
y
,
 
a
n
d


t
h
e
 
t
h
r
e
e
 
c
h
i
l
d
r
e
n
 
o
f
 
i
n
o
d
e
 
1
0
0
5
5
3
'
s
 
f
o
r
k
 
w
e
r
e
 
c
o
m
p
a
r
e
d
 
-
-
 
w
h
i
c
h
 
i
s
 
t
h
e
 
u
s
e
f
u
l
 
p
a
r
t
,


b
e
c
a
u
s
e
 
e
a
c
h
 
h
a
s
 
a
 
*
k
n
o
w
n
*
 
f
i
r
s
t
 
e
x
t
e
n
t
,
 
b
e
c
a
u
s
e
 
t
h
e
 
f
o
r
k
'
s
 
k
e
y
s
 
s
a
y
 
s
o
.




`
`
`
t
e
x
t


d
a
d
d
r
 
5
0
3
1
3
 
 
l
e
v
e
l
 
0
 
n
u
m
r
e
c
s
 
3
0
 
 
 
f
o
r
k
 
k
e
y
 
s
a
y
s
 
i
t
s
 
f
i
r
s
t
 
e
x
t
e
n
t
 
i
s
 
a
t
 
b
l
o
c
k
 
0


d
a
d
d
r
 
5
0
3
1
5
 
 
l
e
v
e
l
 
0
 
n
u
m
r
e
c
s
 
1
5
 
 
 
.
.
.
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
3
0


d
a
d
d
r
 
5
0
3
1
7
 
 
l
e
v
e
l
 
0
 
n
u
m
r
e
c
s
 
1
9
 
 
 
.
.
.
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
4
5


`
`
`




a
n
d
 
a
t
 
o
f
f
s
e
t
 
2
4
 
o
f
 
e
a
c
h
,
 
s
t
e
p
p
i
n
g
 
1
6
 
b
y
t
e
s
:




`
`
`
t
e
x
t


5
0
3
1
3
:
 
 
 
 
 
0
,
 
 
 
5
1
2
,
 
 
1
0
2
4
,
 
.
.
.


5
0
3
1
5
:
 
1
5
3
6
0
,
 
1
5
8
7
2
,
 
1
6
3
8
4
,
 
.
.
.


5
0
3
1
7
:
 
2
3
0
4
0
,
 
2
3
5
5
2
,
 
2
4
0
6
4
,
 
.
.
.


`
`
`




T
h
o
s
e
 
f
i
r
s
t
 
v
a
l
u
e
s
 
a
r
e
 
0
,
 
1
5
3
6
0
 
a
n
d
 
2
3
0
4
0
 
-
-
 
*
*
e
x
a
c
t
l
y
 
t
h
e
 
b
y
t
e
 
o
f
f
s
e
t
s
 
t
h
e
 
f
o
r
k
'
s


k
e
y
s
 
i
m
p
l
y
*
*
 
(
b
l
o
c
k
s
 
0
,
 
3
0
 
a
n
d
 
4
5
 
a
t
 
5
1
2
 
b
y
t
e
s
 
a
p
i
e
c
e
)
.
 
 
T
h
r
e
e
 
l
e
a
v
e
s
,
 
t
h
r
e
e


m
a
t
c
h
e
s
,
 
s
o
 
t
h
i
s
 
i
s
 
e
s
t
a
b
l
i
s
h
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
n
o
t
i
c
e
d
:




*
 
t
h
e
 
m
a
g
i
c
 
i
n
 
a
 
b
-
m
a
p
 
b
l
o
c
k
 
i
s
 
`
B
M
A
P
`
 
(
`
0
x
4
2
4
d
4
1
5
0
`
)
,
 
t
h
e
 
l
e
v
e
l
 
a
n
d
 
r
e
c
o
r
d
 
c
o
u
n
t


 
 
a
r
e
 
t
h
e
 
u
s
u
a
l
 
4
-
b
y
t
e
 
a
n
d
 
2
-
b
y
t
e
 
f
i
e
l
d
s
 
a
t
 
4
 
a
n
d
 
6
,
 
a
n
d
 
t
h
e
 
s
i
b
l
i
n
g
 
f
i
e
l
d
s
 
a
r
e
 
a
t


 
 
8
 
a
n
d
 
1
2
;


*
 
*
*
t
h
e
 
r
e
c
o
r
d
s
 
s
t
a
r
t
 
a
t
 
o
f
f
s
e
t
 
2
4
 
a
n
d
 
a
r
e
 
1
6
 
b
y
t
e
s
 
a
p
a
r
t
*
*
,
 
w
i
t
h
 
t
h
e
 
f
i
l
e
 
o
f
f
s
e
t


 
 
i
n
 
t
h
e
 
*
s
e
c
o
n
d
*
 
e
i
g
h
t
 
b
y
t
e
s
 
o
f
 
e
a
c
h
.




*
*
W
h
a
t
 
i
s
 
n
o
t
 
e
x
p
l
a
i
n
e
d
*
*
 
i
s
 
t
h
e
 
p
a
r
t
n
e
r
 
f
i
e
l
d
.
 
 
I
n
t
e
r
l
e
a
v
e
d
 
w
i
t
h
 
t
h
o
s
e
 
s
t
a
r
t
o
f
f
s


a
r
e
 
v
a
l
u
e
s
 
l
i
k
e
 
`
0
x
0
0
0
0
0
0
1
8
9
1
0
0
0
0
0
1
`
,
 
a
n
d
 
n
o
 
r
e
a
d
i
n
g
 
o
f
 
t
h
e
m
 
i
s
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
 
i
n
 
a


1
3
1
0
7
2
-
b
l
o
c
k
 
i
m
a
g
e
 
-
-
 
`
0
x
0
0
0
0
0
0
1
8
9
1
0
0
0
0
0
1
`
 
i
s
 
6
.
6
 
b
i
l
l
i
o
n
.
 
 
T
h
r
e
e
 
p
o
s
s
i
b
i
l
i
t
i
e
s


r
e
m
a
i
n
 
a
n
d
 
t
h
i
s
 
s
u
i
t
e
 
c
a
n
n
o
t
 
c
u
r
r
e
n
t
l
y
 
c
h
o
o
s
e
 
b
e
t
w
e
e
n
 
t
h
e
m
:




1
.
 
t
h
e
 
p
a
r
t
n
e
r
 
f
i
e
l
d
 
i
s
 
n
o
t
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
 
b
u
t
 
s
o
m
e
t
h
i
n
g
 
t
h
i
s
 
i
m
a
g
e
'
s
 
b
u
i
l
d
e
r


 
 
 
w
r
o
t
e
,
 
w
h
i
c
h
 
i
s
 
a
 
r
e
a
l
 
p
o
s
s
i
b
i
l
i
t
y
 
b
e
c
a
u
s
e
 
`
x
f
s
v
4
.
i
m
g
`
 
i
s
 
*
*
h
a
n
d
-
b
u
i
l
t
*
*
;


2
.
 
t
h
e
 
r
e
c
o
r
d
 
i
s
 
n
o
t
 
1
6
 
b
y
t
e
s
 
a
n
d
 
t
h
e
 
s
e
r
i
e
s
 
a
t
 
o
f
f
s
e
t
 
2
4
 
i
s
 
s
o
m
e
t
h
i
n
g
 
e
l
s
e
 
t
h
a
t


 
 
 
h
a
p
p
e
n
s
 
t
o
 
l
i
n
e
 
u
p
 
w
i
t
h
 
t
h
e
 
f
o
r
k
'
s
 
k
e
y
s
 
t
h
r
e
e
 
t
i
m
e
s
 
o
v
e
r
;


3
.
 
t
h
e
 
f
i
e
l
d
 
i
s
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
 
i
n
 
a
 
u
n
i
t
 
o
r
 
w
i
d
t
h
 
t
h
i
s
 
s
u
i
t
e
 
h
a
s
 
n
o
t
 
i
d
e
n
t
i
f
i
e
d
.




O
p
t
i
o
n
 
2
 
i
s
 
t
h
e
 
o
n
e
 
t
h
a
t
 
w
o
u
l
d
 
i
n
v
a
l
i
d
a
t
e
 
e
v
e
r
y
t
h
i
n
g
 
a
b
o
v
e
,
 
a
n
d
 
i
t
 
i
s
 
c
h
e
a
p
 
t
o


s
e
t
t
l
e
:
 
b
u
i
l
d
 
a
 
s
i
n
g
l
e
 
l
e
a
f
 
w
i
t
h
 
o
n
e
 
r
e
c
o
r
d
 
w
h
o
s
e
 
s
t
a
r
t
o
f
f
 
i
s
 
u
n
a
m
b
i
g
u
o
u
s
,
 
p
o
i
n
t


t
h
e
 
f
o
r
k
 
a
t
 
i
t
,
 
a
n
d
 
a
s
k
 
`
x
f
s
_
r
e
p
a
i
r
`
 
w
h
a
t
 
b
l
o
c
k
 
i
t
 
t
h
i
n
k
s
 
t
h
e
 
e
x
t
e
n
t
 
l
i
v
e
s
 
a
t
.


T
h
a
t
 
e
x
p
e
r
i
m
e
n
t
 
h
a
s
 
n
o
w
 
b
e
e
n
 
r
u
n
,
 
i
n
 
a
 
s
e
t
u
p
 
t
h
a
t
 
i
s
 
*
*
v
e
r
i
f
i
e
d
 
c
o
h
e
r
e
n
t
*
*
,
 
a
n
d
 
i
t


h
a
s
 
e
l
i
m
i
n
a
t
e
d
 
s
i
x
 
c
a
n
d
i
d
a
t
e
 
l
a
y
o
u
t
s
 
w
h
i
l
e
 
p
r
o
d
u
c
i
n
g
 
n
o
 
a
c
c
e
p
t
e
d
 
o
n
e
.




#
#
#
#
 
T
h
e
 
s
e
t
u
p
,
 
a
n
d
 
t
h
e
 
v
e
r
i
f
i
c
a
t
i
o
n
 
t
h
a
t
 
i
t
 
i
s
 
c
o
h
e
r
e
n
t




`
x
f
s
_
d
b
`
 
c
o
n
f
i
r
m
s
 
t
h
e
 
f
o
r
k
 
a
f
t
e
r
 
t
h
e
 
r
e
w
r
i
t
e
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
p
a
r
t
 
t
h
a
t
 
w
a
s
 
w
r
o
n
g


b
e
f
o
r
e
 
a
n
d
 
m
a
d
e
 
e
v
e
r
y
 
e
a
r
l
i
e
r
 
r
e
s
u
l
t
 
m
e
a
n
i
n
g
l
e
s
s
:




`
`
`
t
e
x
t


u
.
b
m
b
t
.
l
e
v
e
l
 
=
 
1
 
 
 
 
 
u
.
b
m
b
t
.
n
u
m
r
e
c
s
 
=
 
3


u
.
b
m
b
t
.
p
t
r
s
[
1
-
3
]
 
=
 
1
:
3
5
8
1
7
 
2
:
3
5
8
1
8
 
3
:
3
5
8
1
9


c
o
r
e
.
n
e
x
t
e
n
t
s
 
=
 
3
 
 
 
 
c
o
r
e
.
n
b
l
o
c
k
s
 
=
 
6
7


`
`
`




T
h
e
 
f
o
r
k
 
i
s
 
u
n
t
o
u
c
h
e
d
 
a
p
a
r
t
 
f
r
o
m
 
i
t
s
 
t
h
r
e
e
 
p
o
i
n
t
e
r
s
,
 
s
o
 
i
t
s
 
k
e
y
s
 
s
t
i
l
l
 
s
a
y
 
i
t
s


c
h
i
l
d
r
e
n
 
b
e
g
i
n
 
a
t
 
b
l
o
c
k
s
 
0
,
 
3
0
 
a
n
d
 
4
5
,
 
a
n
d
 
e
a
c
h
 
n
e
w
 
l
e
a
f
 
h
o
l
d
s
 
o
n
e
 
e
x
t
e
n
t


b
e
g
i
n
n
i
n
g
 
a
t
 
t
h
e
 
m
a
t
c
h
i
n
g
 
f
i
l
e
 
o
f
f
s
e
t
.
 
 
T
h
e
 
b
l
o
c
k
s
 
u
s
e
d
 
a
r
e
 
a
g
1
'
s
 
3
0
4
9
 
t
o
 
3
0
5
1
,


w
h
i
c
h
 
t
h
e
 
b
n
o
 
t
r
e
e
 
r
e
c
o
r
d
s
 
a
s
 
f
r
e
e
,
 
s
o
 
n
o
t
h
i
n
g
 
i
s
 
a
l
l
o
c
a
t
e
d
 
t
w
i
c
e
.




#
#
#
#
 
S
i
x
 
c
a
n
d
i
d
a
t
e
s
,
 
a
l
l
 
r
e
f
u
s
e
d
,
 
e
a
c
h
 
w
i
t
h
 
r
e
p
a
i
r
'
s
 
o
w
n
 
w
o
r
d
s




|
 
r
e
c
o
r
d
 
|
 
w
h
e
r
e
 
|
 
w
h
a
t
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
s
a
i
d
 
|


|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
-
|


|
 
b
l
o
c
k
,
 
t
h
e
n
 
f
i
l
e
 
o
f
f
s
e
t
 
|
 
2
4
 
|
 
`
z
e
r
o
 
l
e
n
g
t
h
 
e
x
t
e
n
t
 
(
o
f
f
 
=
 
6
9
,
 
f
s
b
n
o
 
=
 
4
3
0
1
2
8
9
4
8
7
8
5
9
7
1
2
)
`
 
|


|
 
f
i
l
e
 
o
f
f
s
e
t
,
 
t
h
e
n
 
b
l
o
c
k
 
|
 
2
4
 
|
 
`
b
a
d
 
e
x
t
e
n
t
 
o
v
e
r
f
l
o
w
s
 
-
 
s
t
a
r
t
 
0
,
 
e
n
d
 
3
5
8
1
6
,
 
o
f
f
s
e
t
 
0
`
 
|


|
 
f
i
l
e
 
o
f
f
s
e
t
,
 
b
l
o
c
k
,
 
l
e
n
g
t
h
 
|
 
2
4
 
|
 
`
b
a
d
 
e
x
t
e
n
t
 
o
v
e
r
f
l
o
w
s
 
-
 
s
t
a
r
t
 
0
,
 
e
n
d
 
3
5
8
1
6
,
 
o
f
f
s
e
t
 
0
`
 
|


|
 
f
i
l
e
 
o
f
f
s
e
t
,
 
b
l
o
c
k
,
 
l
e
n
g
t
h
 
|
 
1
6
 
|
 
`
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
4
3
0
1
2
8
9
4
8
7
8
5
9
7
1
2
,
 
o
f
f
s
e
t
 
6
9
`
 
|


|
 
f
i
l
e
 
o
f
f
s
e
t
,
 
l
e
n
g
t
h
,
 
b
l
o
c
k
 
|
 
2
4
 
|
 
`
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
0
,
 
o
f
f
s
e
t
 
0
`
 
|


|
 
b
l
o
c
k
,
 
f
i
l
e
 
o
f
f
s
e
t
,
 
l
e
n
g
t
h
 
|
 
1
6
 
|
 
`
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
0
,
 
o
f
f
s
e
t
 
0
`
 
|




#
#
#
#
 
A
n
d
 
t
h
e
 
t
h
i
n
g
 
t
h
a
t
 
m
a
t
t
e
r
s
 
m
o
s
t
,
 
b
e
c
a
u
s
e
 
i
t
 
i
s
 
n
o
t
 
a
 
c
a
n
d
i
d
a
t
e
 
f
a
i
l
i
n
g




*
*
T
h
e
 
n
u
m
b
e
r
s
 
i
n
 
t
h
o
s
e
 
m
e
s
s
a
g
e
s
 
d
o
 
n
o
t
 
m
a
t
c
h
 
t
h
e
 
b
y
t
e
s
 
t
h
a
t
 
w
e
r
e
 
w
r
i
t
t
e
n
.
*
*


`
3
5
8
1
6
`
 
i
s
 
t
h
e
 
b
l
o
c
k
 
*
b
e
f
o
r
e
*
 
t
h
e
 
f
i
r
s
t
 
l
e
a
f
,
 
a
n
d
 
`
4
3
0
1
2
8
9
4
8
7
8
5
9
7
1
2
`
 
a
p
p
e
a
r
s
 
i
n


t
w
o
 
d
i
f
f
e
r
e
n
t
 
c
a
n
d
i
d
a
t
e
s
 
w
h
o
s
e
 
w
r
i
t
t
e
n
 
b
y
t
e
s
 
d
i
f
f
e
r
;
 
`
o
f
f
 
=
 
0
,
 
f
s
b
n
o
 
=
 
0
`
 
m
e
a
n
s


r
e
p
a
i
r
 
r
e
a
d
 
z
e
r
o
e
s
 
f
r
o
m
 
a
 
p
l
a
c
e
 
t
h
e
 
r
e
c
o
r
d
 
w
a
s
 
n
o
t
 
p
u
t
.
 
 
A
 
r
e
c
o
r
d
 
t
h
a
t
 
i
s
 
*
r
e
a
d
*


p
r
o
d
u
c
e
s
 
a
 
m
e
s
s
a
g
e
 
n
a
m
i
n
g
 
w
h
a
t
 
i
t
 
r
e
a
d
.
 
 
T
h
e
s
e
 
d
o
 
n
o
t
,
 
s
o
 
*
*
r
e
p
a
i
r
 
i
s
 
n
o
t
 
r
e
a
d
i
n
g


t
h
e
 
r
e
c
o
r
d
 
t
h
e
s
e
 
c
o
n
s
t
r
u
c
t
i
o
n
s
 
w
r
i
t
e
 
a
t
 
a
l
l
*
*
,
 
a
n
d
 
t
h
e
 
s
i
x
 
r
e
f
u
s
a
l
s
 
s
a
y
 
t
h
e


c
o
n
s
t
r
u
c
t
u
r
e
s
 
a
r
e
 
w
r
o
n
g
 
r
a
t
h
e
r
 
t
h
a
n
 
t
h
a
t
 
s
i
x
 
l
a
y
o
u
t
s
 
a
r
e
 
w
r
o
n
g
.




A
 
b
e
t
t
e
r
 
w
a
y
 
t
o
 
a
s
k
 
f
o
l
l
o
w
s
 
f
r
o
m
 
t
h
a
t
:
 
i
n
s
t
e
a
d
 
o
f
 
b
u
i
l
d
i
n
g
 
l
e
a
v
e
s
,
 
*
*
p
e
r
t
u
r
b
 
t
h
e


p
r
i
s
t
i
n
e
,
 
a
c
c
e
p
t
e
d
 
o
n
e
s
*
*
,
 
w
h
e
r
e
 
a
 
m
e
s
s
a
g
e
 
c
a
n
 
o
n
l
y
 
b
e
 
a
b
o
u
t
 
t
h
e
 
f
i
e
l
d
 
t
h
a
t
 
w
a
s


c
h
a
n
g
e
d
.
 
 
F
o
u
r
 
o
n
e
-
f
i
e
l
d
 
c
h
a
n
g
e
s
 
t
o
 
l
e
a
f
 
5
0
3
1
3
,
 
e
a
c
h
 
a
c
c
e
p
t
e
d
 
e
x
c
e
p
t
 
t
h
e
 
o
n
e
 
n
a
m
e
d
:




`
`
`
t
e
x
t


o
f
f
s
e
t
 
1
6
 
-
>
 
7
7
7
7
7
7
:
 
 
i
n
 
i
n
o
d
e
 
1
0
0
5
5
3
 
(
d
a
t
a
 
f
o
r
k
)
 
b
m
a
p
 
b
t
r
e
e
 
b
l
o
c
k
 
5
0
3
1
3


o
f
f
s
e
t
 
2
4
 
-
>
 
7
7
7
7
7
7
:
 
 
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
4
3
1
0
0
8
5
5
8
1
3
8
5
0
4
,
 
o
f
f
s
e
t
 
1
5
1
9


o
f
f
s
e
t
 
3
2
 
-
>
 
7
7
7
7
7
7
:
 
 
b
a
d
 
e
x
t
e
n
t
 
o
v
e
r
f
l
o
w
s
 
-
 
s
t
a
r
t
 
0
,
 
e
n
d
 
7
7
7
7
7
6
,
 
o
f
f
s
e
t
 
0


o
f
f
s
e
t
 
4
0
 
-
>
 
7
7
7
7
7
7
:
 
 
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
4
3
1
0
0
8
5
5
8
1
3
8
5
0
6
,
 
o
f
f
s
e
t
 
1
5
1
9


`
`
`




T
w
o
 
o
f
 
t
h
o
s
e
 
a
r
e
 
r
e
a
d
a
b
l
e
 
i
m
m
e
d
i
a
t
e
l
y
,
 
a
n
d
 
t
h
e
y
 
a
r
e
 
t
h
e
 
m
o
s
t
 
u
s
e
f
u
l
 
t
h
i
n
g
 
t
h
i
s
 
l
i
n
e


h
a
s
 
p
r
o
d
u
c
e
d
:




*
 
*
*
o
f
f
s
e
t
 
2
4
 
i
s
 
a
 
f
i
l
e
 
o
f
f
s
e
t
,
 
i
n
 
b
y
t
e
s
.
*
*
 
 
W
r
i
t
i
n
g
 
7
7
7
7
7
7
 
t
h
e
r
e
 
p
r
o
d
u
c
e
d


 
 
`
o
f
f
s
e
t
 
1
5
1
9
`
,
 
a
n
d
 
7
7
7
7
7
7
 
/
 
5
1
2
 
i
s
 
1
5
1
9
.
 
 
S
o
 
t
h
e
 
e
i
g
h
t
 
b
y
t
e
s
 
a
t
 
2
4
 
a
r
e
 
t
h
e


 
 
e
x
t
e
n
t
'
s
 
s
t
a
r
t
,
 
e
x
a
c
t
l
y
 
a
s
 
t
h
e
 
t
h
r
e
e
-
l
e
a
f
 
a
n
a
l
y
s
i
s
 
s
a
i
d
.


*
 
*
*
o
f
f
s
e
t
 
3
2
 
i
s
 
n
o
t
 
t
h
e
 
b
l
o
c
k
 
n
u
m
b
e
r
,
 
a
n
d
 
n
e
i
t
h
e
r
 
i
s
 
o
f
f
s
e
t
 
1
6
.
*
*
 
 
W
r
i
t
i
n
g


 
 
7
7
7
7
7
7
 
a
t
 
3
2
 
p
r
o
d
u
c
e
d
 
`
e
n
d
 
7
7
7
7
7
6
`
 
a
g
a
i
n
s
t
 
a
 
`
s
t
a
r
t
 
0
`
 
—
 
s
o
 
r
e
p
a
i
r
 
r
e
a
d
 
i
t
 
a
s


 
 
s
o
m
e
t
h
i
n
g
 
*
d
e
r
i
v
e
d
 
f
r
o
m
*
 
a
 
s
t
a
r
t
 
a
n
d
 
a
 
l
e
n
g
t
h
,
 
w
i
t
h
 
t
h
e
 
v
a
l
u
e
 
i
t
 
r
e
a
d
 
o
n
e
 
l
e
s
s


 
 
t
h
a
n
 
w
h
a
t
 
w
a
s
 
w
r
i
t
t
e
n
.
 
 
A
n
d
 
t
h
e
 
"
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
"
 
i
t
 
r
e
p
o
r
t
s
,


 
 
`
0
x
1
8
8
0
0
0
0
0
0
c
4
8
8
`
,
 
i
s
 
n
o
t
 
a
n
y
 
e
i
g
h
t
 
b
y
t
e
s
 
o
f
 
t
h
e
 
b
l
o
c
k
:
 
i
t
 
s
h
a
r
e
s
 
`
c
4
8
b
`
 
w
i
t
h


 
 
o
f
f
s
e
t
 
1
6
'
s
 
`
0
0
0
0
c
4
8
b
`
 
(
w
h
i
c
h
 
i
s
 
5
0
3
1
5
,
 
t
h
e
 
d
a
d
d
r
 
o
f
 
t
h
e
 
*
n
e
x
t
*
 
n
o
d
e
)
 
a
n
d


 
 
`
1
8
`
 
w
i
t
h
 
o
f
f
s
e
t
 
3
2
'
s
 
`
0
0
0
0
0
0
1
8
`
,
 
b
u
t
 
i
t
 
i
s
 
n
e
i
t
h
e
r
,
 
s
o
 
i
t
 
i
s
 
*
*
a
s
s
e
m
b
l
e
d
*
*
 
f
r
o
m


 
 
m
o
r
e
 
t
h
a
n
 
o
n
e
 
f
i
e
l
d
.




S
o
 
r
e
p
a
i
r
 
i
s
 
r
e
a
d
i
n
g
 
t
h
i
s
 
l
e
a
f
,
 
i
t
 
r
e
a
d
s
 
t
h
e
 
s
t
a
r
t
 
a
t
 
o
f
f
s
e
t
 
2
4
,
 
a
n
d
 
t
h
e
 
b
l
o
c
k


n
u
m
b
e
r
 
i
t
 
r
e
p
o
r
t
s
 
i
s
 
a
 
c
o
m
p
o
s
i
t
e
 
i
t
 
d
e
r
i
v
e
s
 
r
a
t
h
e
r
 
t
h
a
n
 
a
 
f
i
e
l
d
 
i
t
 
r
e
a
d
s
.
 
 
T
h
a
t
 
i
s


a
 
r
e
a
l
 
l
e
a
d
 
—
 
a
n
d
 
i
t
 
i
s
 
a
s
 
f
a
r
 
a
s
 
t
h
i
s
 
l
i
n
e
 
g
o
e
s
.
 
 
G
u
e
s
s
i
n
g
 
a
 
l
a
y
o
u
t
 
f
r
o
m
 
a
 
n
u
m
b
e
r


t
h
a
t
 
i
s
 
d
e
m
o
n
s
t
r
a
b
l
y
 
a
 
c
o
m
p
o
s
i
t
e
 
i
s
 
e
x
a
c
t
l
y
 
t
h
e
 
f
a
i
l
u
r
e
 
m
o
d
e
 
t
h
i
s
 
d
o
c
u
m
e
n
t
 
n
o
w


e
x
i
s
t
s
 
t
o
 
p
r
e
v
e
n
t
:
 
a
 
p
l
a
u
s
i
b
l
e
 
a
n
s
w
e
r
,
 
a
s
s
e
m
b
l
e
d
 
f
r
o
m
 
t
h
e
 
w
r
o
n
g
 
p
i
e
c
e
s
,
 
t
h
a
t
 
w
o
u
l
d


h
a
v
e
 
b
e
c
o
m
e
 
a
n
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
.




T
h
a
t
 
i
s
o
l
a
t
i
o
n
 
s
t
e
p
 
w
a
s
 
t
h
e
n
 
t
a
k
e
n
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
 
r
e
a
l
 
a
d
v
a
n
c
e
 
o
f
 
t
h
i
s
 
l
i
n
e
.


P
e
r
t
u
r
b
i
n
g
 
t
h
e
 
*
p
r
i
s
t
i
n
e
*
 
l
e
a
v
e
s
 
o
n
e
 
e
i
g
h
t
-
b
y
t
e
 
f
i
e
l
d
 
a
t
 
a
 
t
i
m
e
:




`
`
`
t
e
x
t


o
f
f
s
e
t
 
1
2
 
-
>
 
7
7
7
7
7
7
:
 
 
n
o
 
e
x
t
e
n
t
 
c
o
m
p
l
a
i
n
t
 
 
 
 
 
 
(
a
 
s
i
b
l
i
n
g
 
f
i
e
l
d
,
 
n
o
t
 
a
 
r
e
c
o
r
d
)


o
f
f
s
e
t
 
2
0
 
-
>
 
7
7
7
7
7
7
:
 
 
c
o
r
r
e
c
t
i
n
g
 
n
e
x
t
e
n
t
s
 
o
n
l
y


o
f
f
s
e
t
 
2
8
 
-
>
 
7
7
7
7
7
7
:
 
 
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
1
5
9
2
8
8
8
4
5
6
,
 
o
f
f
s
e
t
 
0


o
f
f
s
e
t
 
3
6
 
-
>
 
7
7
7
7
7
7
:
 
 
z
e
r
o
 
l
e
n
g
t
h
 
e
x
t
e
n
t
 
(
o
f
f
 
=
 
0
,
 
f
s
b
n
o
 
=
 
4
9
1
5
2
)


o
f
f
s
e
t
 
4
4
 
-
>
 
7
7
7
7
7
7
:
 
 
n
o
 
e
x
t
e
n
t
 
c
o
m
p
l
a
i
n
t


`
`
`




T
w
o
 
t
h
i
n
g
s
 
f
o
l
l
o
w
,
 
a
n
d
 
t
h
e
 
s
e
c
o
n
d
 
i
s
 
t
h
e
 
o
n
e
 
w
o
r
t
h
 
h
a
v
i
n
g
:




*
 
*
*
T
h
e
 
r
e
p
o
r
t
e
d
 
b
l
o
c
k
 
n
u
m
b
e
r
 
i
s
 
a
 
c
o
m
p
o
s
i
t
e
*
*
,
 
a
n
d
 
t
h
e
 
p
r
o
b
e
s
 
s
h
o
w
 
w
h
i
c
h
 
b
y
t
e
s
 
g
o


 
 
i
n
t
o
 
i
t
:
 
p
a
t
c
h
i
n
g
 
t
h
e
 
l
o
w
 
h
a
l
f
 
o
f
 
o
f
f
s
e
t
 
3
2
'
s
 
f
i
e
l
d
 
t
u
r
n
s


 
 
`
0
x
1
8
8
0
0
0
0
0
0
c
4
8
8
`
 
i
n
t
o
 
`
1
5
9
2
8
8
8
4
5
6
`
,
 
a
n
d
 
p
a
t
c
h
i
n
g
 
t
h
e
 
h
i
g
h
 
h
a
l
f
 
t
u
r
n
s
 
i
t
 
i
n
t
o


 
 
`
4
9
1
5
2
`
.
 
 
B
o
t
h
 
c
h
a
n
g
e
s
 
m
o
v
e
 
t
h
e
 
r
e
p
o
r
t
e
d
 
n
u
m
b
e
r
,
 
a
n
d
 
b
o
t
h
 
l
e
a
v
e
 
t
h
e
 
e
x
t
e
n
t
'
s


 
 
*
s
t
a
r
t
*
 
a
t
 
0
 
—
 
s
o
 
t
h
e
 
f
i
e
l
d
 
a
t
 
o
f
f
s
e
t
 
3
2
 
p
a
r
t
i
c
i
p
a
t
e
s
 
i
n
 
t
h
e
 
n
u
m
b
e
r
 
r
e
p
a
i
r
 
r
e
p
o
r
t
s


 
 
w
i
t
h
o
u
t
 
b
e
i
n
g
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
 
i
t
s
e
l
f
.


*
 
*
*
P
e
r
t
u
r
b
i
n
g
 
a
 
p
r
i
s
t
i
n
e
 
l
e
a
f
 
r
e
a
c
h
e
s
 
t
h
e
 
r
e
c
o
r
d
;
 
c
o
n
s
t
r
u
c
t
i
n
g
 
a
 
n
e
w
 
o
n
e
 
d
o
e
s


 
 
n
o
t
.
*
*
 
 
T
h
a
t
 
i
s
 
t
h
e
 
m
e
t
h
o
d
o
l
o
g
i
c
a
l
 
c
o
r
r
e
c
t
i
o
n
,
 
a
n
d
 
i
t
 
i
s
 
w
h
y
 
s
i
x
 
c
o
n
s
t
r
u
c
t
e
d


 
 
c
a
n
d
i
d
a
t
e
s
 
s
a
i
d
 
n
o
t
h
i
n
g
:
 
t
h
e
y
 
w
e
r
e
 
n
o
t
 
b
e
i
n
g
 
r
e
a
d
,
 
a
n
d
 
a
 
t
a
b
l
e
 
o
f
 
t
h
e
i
r
 
r
e
f
u
s
a
l
s


 
 
w
o
u
l
d
 
h
a
v
e
 
r
e
a
d
 
a
s
 
e
v
i
d
e
n
c
e
 
a
b
o
u
t
 
t
h
e
 
f
o
r
m
a
t
 
w
h
e
n
 
i
t
 
w
a
s
 
e
v
i
d
e
n
c
e
 
a
b
o
u
t
 
t
h
e


 
 
c
o
n
s
t
r
u
c
t
i
o
n
.
 
 
P
e
r
t
u
r
b
i
n
g
 
s
o
m
e
t
h
i
n
g
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
 
a
l
r
e
a
d
y
 
a
c
c
e
p
t
s
 
r
e
m
o
v
e
s
 
t
h
a
t


 
 
e
n
t
i
r
e
 
c
l
a
s
s
 
o
f
 
m
i
s
t
a
k
e
,
 
b
e
c
a
u
s
e
 
t
h
e
 
o
n
l
y
 
d
i
f
f
e
r
e
n
c
e
 
i
s
 
t
h
e
 
f
i
e
l
d
 
u
n
d
e
r
 
t
e
s
t
.




P
u
s
h
i
n
g
 
t
h
a
t
 
o
n
e
 
s
t
e
p
 
f
u
r
t
h
e
r
,
 
w
i
t
h
 
a
 
v
a
l
u
e
 
c
h
o
s
e
n
 
s
o
 
t
h
a
t
 
e
v
e
r
y
 
b
y
t
e
 
o
f
 
i
t
 
i
s


u
n
m
i
s
t
a
k
a
b
l
e
,
 
a
t
t
r
i
b
u
t
e
s
 
e
a
c
h
 
f
i
e
l
d
 
t
o
 
w
h
a
t
 
r
e
p
a
i
r
 
t
h
e
n
 
r
e
p
o
r
t
s
:




`
`
`
t
e
x
t


l
e
a
f
+
1
6
 
=
 
1
1
2
2
3
3
4
4
5
5
6
6
7
7
8
8
:
 
 
i
n
 
i
n
o
d
e
 
1
0
0
5
5
3
 
(
d
a
t
a
 
f
o
r
k
)
 
b
m
a
p
 
b
t
r
e
e
 
b
l
o
c
k
 
5
0
3
1
3


 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
a
n
d
 
n
o
 
e
x
t
e
n
t
 
c
o
m
p
l
a
i
n
t
 
a
t
 
a
l
l


l
e
a
f
+
2
4
 
=
 
1
1
2
2
3
3
4
4
5
5
6
6
7
7
8
8
:
 
 
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
<
c
o
m
p
o
s
i
t
e
>
,
 
o
f
f
s
e
t
 
1
5
1
9


l
e
a
f
+
3
2
 
=
 
0
0
0
0
0
0
0
0
0
0
a
b
c
d
e
f
:
 
 
b
a
d
 
e
x
t
e
n
t
 
o
v
e
r
f
l
o
w
s
 
-
 
s
t
a
r
t
 
5
,
 
e
n
d
 
7
7
3
6
1
9
,
 
o
f
f
s
e
t
 
0


l
e
a
f
+
3
2
 
=
 
a
a
b
b
c
c
d
d
e
e
f
f
0
0
1
1
:
 
 
b
a
d
 
e
x
t
e
n
t
 
o
v
e
r
f
l
o
w
s
 
-
 
s
t
a
r
t
 
5
8
6
6
3
6
1
6
4
6
9
6
7
,
 
e
n
d
 
.
.
.


l
e
a
f
+
4
0
 
=
 
1
1
2
2
3
3
4
4
5
5
6
6
7
7
8
8
:
 
 
b
a
d
 
e
x
t
e
n
t
 
s
t
a
r
t
i
n
g
 
b
l
o
c
k
 
n
u
m
b
e
r
 
<
c
o
m
p
o
s
i
t
e
 
+
 
2
>
,
 
o
f
f
s
e
t
 
1
5
1
9


`
`
`




R
e
a
d
 
a
s
 
a
t
t
r
i
b
u
t
i
o
n
s
 
r
a
t
h
e
r
 
t
h
a
n
 
a
s
 
n
u
m
b
e
r
s
:




*
 
*
*
O
f
f
s
e
t
 
1
6
 
i
s
 
n
o
t
 
a
 
f
i
e
l
d
 
r
e
p
a
i
r
 
v
a
l
i
d
a
t
e
s
.
*
*
 
 
A
n
 
u
n
m
i
s
t
a
k
a
b
l
e
 
v
a
l
u
e
 
t
h
e
r
e


 
 
c
h
a
n
g
e
s
 
n
o
t
h
i
n
g
 
a
b
o
u
t
 
t
h
e
 
e
x
t
e
n
t
s
.
 
 
I
t
s
 
p
r
i
s
t
i
n
e
 
v
a
l
u
e
 
i
n
 
t
h
e
s
e
 
l
e
a
v
e
s
 
h
a
p
p
e
n
s


 
 
t
o
 
b
e
 
`
0
x
0
0
0
0
0
0
0
0
0
0
0
0
c
4
8
b
`
 
=
 
5
0
3
1
5
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
*
n
e
x
t
 
n
o
d
e
'
s
*
 
d
a
d
d
r
 
—
 
a
n
d
 
t
h
e


 
 
t
h
r
e
e
 
l
e
a
v
e
s
 
a
r
e
 
a
t
 
c
o
n
s
e
c
u
t
i
v
e
 
o
d
d
 
d
a
d
d
r
s
,
 
s
o
 
t
h
a
t
 
w
a
s
 
a
l
w
a
y
s
 
a
 
c
o
i
n
c
i
d
e
n
c
e
 
o
f


 
 
a
l
l
o
c
a
t
i
o
n
 
r
a
t
h
e
r
 
t
h
a
n
 
a
 
f
i
e
l
d
.
 
 
T
h
e
 
e
a
r
l
i
e
r
 
r
e
a
d
i
n
g
 
t
h
a
t
 
t
r
e
a
t
e
d
 
i
t
 
a
s
 
t
h
e
 
f
i
r
s
t


 
 
e
x
t
e
n
t
'
s
 
b
l
o
c
k
 
w
a
s
 
w
r
o
n
g
,
 
a
n
d
 
t
h
i
s
 
i
s
 
w
h
a
t
 
k
i
l
l
e
d
 
i
t
.


*
 
*
*
O
f
f
s
e
t
 
2
4
 
i
s
 
t
h
e
 
e
x
t
e
n
t
'
s
 
s
t
a
r
t
,
 
i
n
 
b
y
t
e
s
*
*
 
—
 
c
o
n
f
i
r
m
e
d
 
e
a
r
l
i
e
r
,
 
a
n
d
 
c
o
n
f
i
r
m
e
d


 
 
a
g
a
i
n
 
h
e
r
e
:
 
7
7
7
7
7
7
 
/
 
5
1
2
 
i
s
 
t
h
e
 
1
5
1
9
 
i
t
 
r
e
p
o
r
t
e
d
.


*
 
*
*
O
f
f
s
e
t
 
3
2
 
i
s
 
n
o
t
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
.
*
*
 
 
P
a
t
c
h
i
n
g
 
i
t
 
m
o
v
e
s
 
t
h
e
 
r
e
p
o
r
t
e
d
 
*
s
t
a
r
t
*
 
a
n
d


 
 
*
e
n
d
*
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
a
 
l
e
n
g
t
h
 
d
o
e
s
 
a
n
d
 
i
s
 
n
o
t
 
w
h
a
t
 
a
 
b
l
o
c
k
 
d
o
e
s
.


*
 
*
*
O
f
f
s
e
t
 
4
0
 
b
e
h
a
v
e
s
 
l
i
k
e
 
a
 
s
t
a
r
t
*
*
,
 
g
i
v
i
n
g
 
t
h
e
 
s
a
m
e
 
s
h
a
p
e
 
o
f
 
m
e
s
s
a
g
e
 
a
s
 
o
f
f
s
e
t
 
2
4


 
 
w
i
t
h
 
t
h
e
 
c
o
m
p
o
s
i
t
e
 
d
i
f
f
e
r
i
n
g
 
b
y
 
t
w
o
.




*
*
A
n
d
 
t
h
e
 
s
t
r
i
d
e
 
d
o
e
s
 
n
o
t
 
f
i
t
*
*
 
—
 
s
t
a
r
t
s
 
a
p
p
e
a
r
 
e
v
e
r
y
 
s
i
x
t
e
e
n
 
b
y
t
e
s
,
 
s
o
 
a


s
i
x
t
e
e
n
-
b
y
t
e
 
r
e
c
o
r
d
 
w
o
u
l
d
 
b
e
 
`
{
s
t
a
r
t
,
 
l
e
n
g
t
h
}
`
 
a
n
d
 
w
o
u
l
d
 
h
o
l
d
 
n
o
 
b
l
o
c
k
 
n
u
m
b
e
r
 
a
t


a
l
l
,
 
w
h
i
c
h
 
n
o
 
m
a
p
p
i
n
g
 
r
e
c
o
r
d
 
c
a
n
 
d
o
.
 
 
S
o
 
t
h
e
 
n
e
x
t
 
p
r
o
b
e
 
w
a
s
 
t
o
 
r
e
a
d
 
t
h
e
 
r
e
g
i
o
n
 
a
s


f
o
u
r
-
b
y
t
e
 
w
o
r
d
s
 
a
n
d
 
p
e
r
t
u
r
b
 
e
a
c
h
,
 
i
n
s
t
e
a
d
 
o
f
 
a
s
 
6
4
-
b
i
t
 
v
a
l
u
e
s
.




T
h
a
t
 
p
r
o
b
e
 
a
n
s
w
e
r
s
 
i
t
,
 
i
n
 
t
h
e
 
c
h
e
c
k
e
r
'
s
 
o
w
n
 
v
o
c
a
b
u
l
a
r
y
 
r
a
t
h
e
r
 
t
h
a
n
 
b
y
 
i
n
f
e
r
e
n
c
e
.


`
x
f
s
_
r
e
p
a
i
r
`
 
p
r
i
n
t
s
 
b
-
m
a
p
 
r
e
c
o
r
d
s
 
a
s
:




`
`
`
t
e
x
t


b
m
a
p
 
r
e
c
 
o
u
t
 
o
f
 
o
r
d
e
r
,
 
i
n
o
d
e
 
1
0
0
5
5
3
 
e
n
t
r
y
 
1
 
[
o
 
s
 
c
]
 
[
1
 
5
0
3
1
4
 
1
]
,
 
0
 
[
1
3
3
2
4
8
4
1
6
8
7
9
7
3
8
8
8
 
5
0
3
1
2
 
1
]


`
`
`




*
*
`
[
o
 
s
 
c
]
`
:
 
o
f
f
s
e
t
,
 
s
t
a
r
t
 
b
l
o
c
k
,
 
c
o
u
n
t
.
*
*
 
 
E
n
t
r
i
e
s
 
a
r
e
 
n
u
m
b
e
r
e
d
 
f
r
o
m
 
z
e
r
o
,
 
a
n
d


p
e
r
t
u
r
b
i
n
g
 
o
n
e
 
4
-
b
y
t
e
 
w
o
r
d
 
m
a
k
e
s
 
o
n
e
 
e
n
t
r
y
'
s
 
r
e
c
o
r
d
 
u
n
o
r
d
e
r
a
b
l
e
 
a
n
d
 
p
r
i
n
t
s
 
t
h
e


w
h
o
l
e
 
o
f
 
i
t
.
 
 
S
o
 
t
h
e
 
l
a
y
o
u
t
 
i
s
 
r
e
a
d
 
o
f
f
 
t
h
e
 
m
e
s
s
a
g
e
 
r
a
t
h
e
r
 
t
h
a
n
 
g
u
e
s
s
e
d
:




`
`
`
t
e
x
t


 
 
o
f
f
s
e
t
 
2
4
 
+
 
1
6
n
 
 
 
o
 
 
 
t
h
e
 
e
x
t
e
n
t
'
s
 
f
i
l
e
 
o
f
f
s
e
t


 
 
o
f
f
s
e
t
 
2
8
 
+
 
1
6
n
 
 
 
s
 
 
 
i
t
s
 
f
i
r
s
t
 
d
a
t
a
 
b
l
o
c
k


 
 
o
f
f
s
e
t
 
3
2
 
+
 
1
6
n
 
 
 
c
 
 
 
i
t
s
 
l
e
n
g
t
h


`
`
`




w
h
i
c
h
 
i
s
 
t
h
r
e
e
 
4
-
b
y
t
e
 
f
i
e
l
d
s
 
i
n
 
a
 
1
6
-
b
y
t
e
 
r
e
c
o
r
d
,
 
a
n
d
 
i
t
 
i
s
 
w
h
y
 
e
v
e
r
y
 
r
e
a
d
i
n
g


t
h
a
t
 
t
r
e
a
t
e
d
 
t
h
e
 
e
i
g
h
t
 
b
y
t
e
s
 
a
t
 
o
f
f
s
e
t
 
3
2
 
a
s
 
a
 
6
4
-
b
i
t
 
v
a
l
u
e
 
f
a
i
l
e
d
:
 
i
t
 
s
p
a
n
s
 
t
w
o


f
i
e
l
d
s
 
a
n
d
 
a
 
h
a
l
f
.




*
*
A
n
d
 
t
h
e
 
o
f
f
s
e
t
s
 
a
r
e
 
i
n
 
b
l
o
c
k
s
,
 
n
o
t
 
b
y
t
e
s
*
*
 
—
 
e
n
t
r
i
e
s
 
0
,
 
1
,
 
2
,
 
3
,
 
e
a
c
h
 
o
n
e
 
b
l
o
c
k


l
o
n
g
,
 
w
i
t
h
 
t
h
e
i
r
 
d
a
t
a
 
b
l
o
c
k
s
 
a
t
 
5
0
3
1
2
,
 
5
0
3
1
4
,
 
5
0
3
1
6
,
 
5
0
3
1
8
 
i
n
t
e
r
l
e
a
v
e
d
 
w
i
t
h
 
t
h
e


t
r
e
e
 
n
o
d
e
s
 
a
t
 
5
0
3
1
3
,
 
5
0
3
1
5
,
 
5
0
3
1
7
.
 
 
T
h
a
t
 
i
s
 
w
h
a
t
 
a
 
b
u
i
l
d
e
r
 
a
l
l
o
c
a
t
i
n
g
 
a
 
d
a
t
a
 
b
l
o
c
k


a
n
d
 
a
 
t
r
e
e
 
n
o
d
e
 
a
l
t
e
r
n
a
t
e
l
y
 
p
r
o
d
u
c
e
s
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
 
l
a
s
t
 
o
f
 
t
h
e
 
v
a
l
u
e
s
 
t
h
i
s
 
l
i
n
e


f
i
r
s
t
 
r
e
a
d
 
a
s
 
`
0
x
0
0
0
0
0
0
1
8
9
1
0
0
0
0
0
1
`
 
a
n
d
 
c
o
u
l
d
 
n
o
t
 
a
c
c
o
u
n
t
 
f
o
r
:
 
`
0
0
 
0
0
 
0
0
 
1
8
`
 
i
s
 
a


d
i
f
f
e
r
e
n
t
 
f
i
e
l
d
 
f
r
o
m
 
`
9
1
 
0
0
 
0
0
 
0
1
`
,
 
a
n
d
 
b
o
t
h
 
w
e
r
e
 
b
e
i
n
g
 
r
e
a
d
 
a
s
 
o
n
e
 
n
u
m
b
e
r
.




S
o
 
t
h
e
 
l
a
y
o
u
t
 
i
s
:




|
 
o
f
f
s
e
t
 
|
 
f
i
e
l
d
 
|
 
n
o
t
e
 
|


|
:
-
-
-
-
-
-
-
|
:
-
-
-
-
-
-
|
:
-
-
-
-
-
|


|
 
0
 
|
 
`
B
M
A
P
`
 
(
`
0
x
4
2
4
d
4
1
5
0
`
)
 
|
 
m
e
a
s
u
r
e
d
 
i
n
 
t
h
r
e
e
 
l
e
a
v
e
s
 
|


|
 
4
 
|
 
l
e
v
e
l
 
(
u
1
6
)
,
 
r
e
c
o
r
d
 
c
o
u
n
t
 
(
u
1
6
)
 
|
 
|


|
 
8
,
 
1
2
 
|
 
l
e
f
t
 
a
n
d
 
r
i
g
h
t
 
s
i
b
l
i
n
g
 
|
 
|


|
 
1
6
 
|
 
n
o
t
h
i
n
g
 
r
e
p
a
i
r
 
v
a
l
i
d
a
t
e
s
 
|
 
p
r
i
s
t
i
n
e
 
c
o
n
t
e
n
t
s
 
h
e
r
e
 
a
r
e
 
t
h
e
 
n
e
x
t
 
n
o
d
e
'
s
 
d
a
d
d
r
,
 
b
y
 
c
o
i
n
c
i
d
e
n
c
e
 
|


|
 
2
4
 
+
 
1
6
n
 
|
 
n
o
t
 
a
t
t
r
i
b
u
t
e
d
 
|
 
c
o
n
s
t
a
n
t
 
a
c
r
o
s
s
 
e
v
e
r
y
 
r
e
c
o
r
d
:
 
`
0
0
0
0
0
0
0
0
`
 
|


|
 
2
8
 
+
 
1
6
n
 
|
 
t
h
e
 
e
x
t
e
n
t
'
s
 
f
i
l
e
 
o
f
f
s
e
t
,
 
i
n
 
*
*
b
y
t
e
s
*
*
 
|
 
m
e
a
s
u
r
e
d
 
|


|
 
3
2
 
+
 
1
6
n
 
|
 
n
o
t
 
a
t
t
r
i
b
u
t
e
d
 
|
 
c
o
n
s
t
a
n
t
 
a
c
r
o
s
s
 
e
v
e
r
y
 
r
e
c
o
r
d
:
 
`
0
0
0
0
0
0
1
8
`
 
|


|
 
3
6
 
+
 
1
6
n
 
|
 
`
(
e
n
t
r
y
 
<
<
 
1
6
)
 
\
|
 
l
e
n
g
t
h
 
i
n
 
b
l
o
c
k
s
`
 
|
 
m
e
a
s
u
r
e
d
 
a
c
r
o
s
s
 
a
l
l
 
t
h
r
e
e
 
l
e
a
v
e
s
 
|




T
h
e
 
f
o
u
r
t
h
 
w
o
r
d
 
r
e
s
o
l
v
e
s
 
i
n
t
o
 
t
w
o
 
h
a
l
v
e
s
 
o
n
c
e
 
t
h
e
 
t
h
r
e
e
 
l
e
a
v
e
s
 
a
r
e
 
r
e
a
d
 
t
o
g
e
t
h
e
r
,


w
h
i
c
h
 
i
s
 
w
h
a
t
 
i
t
 
t
o
o
k
:




`
`
`
t
e
x
t


l
e
a
f
 
1
 
(
e
n
t
r
i
e
s
 
 
0
.
.
2
9
)
 
 
f
i
r
s
t
 
o
=
0
 
 
 
 
 
l
a
s
t
 
o
=
1
4
8
4
8
 
 
 
h
i
g
h
 
0
x
9
1
0
0
 
-
>
 
0
x
9
8
4
0


l
e
a
f
 
2
 
(
e
n
t
r
i
e
s
 
3
0
.
.
4
4
)
 
 
f
i
r
s
t
 
o
=
1
5
3
6
0
 
 
l
a
s
t
 
o
=
2
2
5
2
8
 
 
 
h
i
g
h
 
0
x
9
8
8
0
 
-
>
 
0
x
9
c
0
0


l
e
a
f
 
3
 
(
e
n
t
r
i
e
s
 
4
5
.
.
6
3
)
 
 
f
i
r
s
t
 
o
=
2
3
0
4
0
 
 
l
a
s
t
 
o
=
3
2
2
5
6
 
 
 
h
i
g
h
 
0
x
9
c
4
0
 
-
>
 
0
x
a
0
c
0


`
`
`




T
h
e
 
l
o
w
 
h
a
l
f
 
i
s
 
`
0
0
0
1
`
 
i
n
 
e
v
e
r
y
 
r
e
c
o
r
d
 
o
f
 
e
v
e
r
y
 
l
e
a
f
,
 
a
n
d
 
t
h
e
 
e
x
t
e
n
t
s
 
a
r
e
 
o
n
e
 
b
l
o
c
k


e
a
c
h
,
 
s
o
 
i
t
 
i
s
 
t
h
e
 
*
*
l
e
n
g
t
h
 
i
n
 
b
l
o
c
k
s
*
*
.
 
 
T
h
e
 
h
i
g
h
 
h
a
l
f
 
a
d
v
a
n
c
e
s
 
b
y
 
`
0
x
4
0
`
 
—
 
6
4
 
—


p
e
r
 
e
n
t
r
y
,
 
a
n
d
 
6
4
 
i
s
 
e
x
a
c
t
l
y
 
t
h
e
 
n
u
m
b
e
r
 
o
f
 
e
n
t
r
i
e
s
 
i
n
 
t
h
e
 
f
i
l
e
,
 
r
u
n
n
i
n
g
 
f
r
o
m


`
0
x
9
1
0
0
`
 
o
n
 
e
n
t
r
y
 
0
 
t
o
 
`
0
x
a
0
c
0
`
 
o
n
 
e
n
t
r
y
 
6
3
 
w
i
t
h
o
u
t
 
a
 
b
r
e
a
k
 
a
t
 
e
i
t
h
e
r
 
l
e
a
f


b
o
u
n
d
a
r
y
.
 
 
S
o
 
i
t
 
i
s
 
a
n
 
*
*
e
n
t
r
y
 
c
o
u
n
t
e
r
*
*
,
 
n
o
t
 
a
 
b
l
o
c
k
 
n
u
m
b
e
r
:
 
i
t
 
k
e
e
p
s
 
c
o
u
n
t
i
n
g


a
c
r
o
s
s
 
n
o
d
e
s
,
 
w
h
i
c
h
 
n
o
 
b
l
o
c
k
 
n
u
m
b
e
r
 
w
o
u
l
d
.




*
*
A
n
d
 
t
h
a
t
 
i
s
 
t
h
e
 
f
i
n
d
i
n
g
 
t
h
a
t
 
c
l
o
s
e
s
 
t
h
e
 
s
e
a
r
c
h
.
*
*
 
 
T
h
e
 
l
e
a
f
'
s
 
r
e
c
o
r
d
s
 
h
o
l
d
 
t
h
e


f
i
l
e
 
o
f
f
s
e
t
 
a
n
d
 
t
h
e
 
l
e
n
g
t
h
,
 
a
n
d
 
t
h
e
y
 
d
o
 
*
*
n
o
t
*
*
 
h
o
l
d
 
t
h
e
 
d
a
t
a
 
b
l
o
c
k
 
—
 
t
h
e
 
t
w
o


c
o
n
s
t
a
n
t
 
w
o
r
d
s
 
a
r
e
 
n
o
t
 
i
t
,
 
a
n
d
 
t
h
e
 
f
o
u
r
t
h
 
w
o
r
d
 
i
s
 
a
 
c
o
u
n
t
e
r
.
 
 
S
o
 
t
h
e
 
m
a
p
p
i
n
g
'
s


b
l
o
c
k
 
f
o
r
 
t
h
e
s
e
 
f
i
l
e
s
 
l
i
v
e
s
 
s
o
m
e
w
h
e
r
e
 
t
h
i
s
 
h
a
s
 
n
o
t
 
l
o
o
k
e
d
,
 
a
n
d
 
t
h
e
 
r
e
a
d
e
r
 
c
a
n
n
o
t


b
e
 
c
o
m
p
l
e
t
e
d
 
b
y
 
r
e
a
d
i
n
g
 
t
h
e
 
r
e
c
o
r
d
 
d
i
f
f
e
r
e
n
t
l
y
;
 
t
h
e
r
e
 
i
s
 
n
o
t
h
i
n
g
 
e
l
s
e
 
i
n
 
i
t
 
t
o


r
e
a
d
.
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
c
a
n
 
c
o
m
p
u
t
e
 
a
 
b
l
o
c
k
 
f
o
r
 
e
v
e
r
y
 
e
n
t
r
y
,
 
s
o
 
i
t
 
h
a
s
 
a
 
s
o
u
r
c
e
,
 
a
n
d


f
i
n
d
i
n
g
 
i
t
 
i
s
 
t
h
e
 
n
e
x
t
 
q
u
e
s
t
i
o
n
 
r
a
t
h
e
r
 
t
h
a
n
 
r
e
-
e
x
a
m
i
n
i
n
g
 
t
h
e
 
r
e
c
o
r
d
.




T
h
e
 
o
f
f
s
e
t
s
 
a
r
e
 
c
o
n
f
i
r
m
e
d
 
a
c
r
o
s
s
 
a
l
l
 
t
h
r
e
e
 
l
e
a
v
e
s
 
a
n
d
 
a
r
e
 
c
o
n
t
i
n
u
o
u
s
 
f
r
o
m
 
0
 
t
o


3
2
2
5
6
 
f
o
r
 
a
 
3
2
7
6
8
-
b
y
t
e
 
f
i
l
e
,
 
w
i
t
h
 
o
n
e
 
e
n
t
r
y
 
p
e
r
 
5
1
2
-
b
y
t
e
 
b
l
o
c
k
 
—
 
s
o
 
t
h
e
 
s
e
c
o
n
d


w
o
r
d
 
i
s
 
t
h
e
 
e
x
t
e
n
t
'
s
 
f
i
l
e
 
o
f
f
s
e
t
 
i
n
 
b
y
t
e
s
,
 
a
n
d
 
t
h
e
 
o
f
f
s
e
t
s
 
`
x
f
s
_
r
e
p
a
i
r
`
 
*
p
r
i
n
t
s
*


(
0
,
 
1
,
 
2
,
 
3
)
 
a
r
e
 
t
h
o
s
e
 
b
y
t
e
 
o
f
f
s
e
t
s
 
d
i
v
i
d
e
d
 
b
y
 
t
h
e
 
b
l
o
c
k
 
s
i
z
e
.
 
 
A
 
r
e
a
d
e
r
 
t
h
a
t
 
t
o
o
k


t
h
e
 
p
r
i
n
t
e
d
 
f
o
r
m
 
f
o
r
 
t
h
e
 
s
t
o
r
e
d
 
o
n
e
 
w
o
u
l
d
 
b
e
 
o
u
t
 
b
y
 
a
 
f
a
c
t
o
r
 
o
f
 
t
h
e
 
b
l
o
c
k
 
s
i
z
e
,


w
h
i
c
h
 
i
s
 
t
h
e
 
s
a
m
e
 
t
r
a
p
 
a
s
 
t
h
e
 
c
o
m
p
o
s
i
t
e
:
 
t
h
e
 
c
h
e
c
k
e
r
'
s
 
n
u
m
b
e
r
s
 
a
r
e
 
d
e
c
o
d
e
d
 
v
a
l
u
e
s
,


n
o
t
 
t
h
e
 
b
y
t
e
s
 
o
n
 
t
h
e
 
d
i
s
k
.




*
*
S
o
 
t
h
e
 
r
e
c
o
r
d
 
i
s
 
`
{
u
n
u
s
e
d
,
 
o
f
f
s
e
t
 
i
n
 
b
y
t
e
s
,
 
u
n
u
s
e
d
,
 
e
n
t
r
y
 
c
o
u
n
t
e
r
 
a
n
d
 
l
e
n
g
t
h
}
`
*
*
,


a
n
d
 
w
h
e
r
e
 
a
n
 
e
x
t
e
n
t
'
s
 
b
l
o
c
k
 
c
o
m
e
s
 
f
r
o
m
 
i
s
 
t
h
e
 
o
p
e
n
 
q
u
e
s
t
i
o
n
.




#
#
#
 
T
h
e
 
b
l
o
c
k
 
i
s
 
n
o
t
 
i
n
 
t
h
e
 
i
m
a
g
e
 
e
i
t
h
e
r
,
 
a
n
d
 
t
h
a
t
 
i
s
 
w
h
y
 
t
h
i
s
 
c
a
n
n
o
t
 
b
e
 
m
e
a
s
u
r
e
d
 
h
e
r
e




T
w
o
 
m
o
r
e
 
e
x
p
e
r
i
m
e
n
t
s
,
 
a
n
d
 
t
h
e
y
 
s
e
t
t
l
e
 
i
t
.




*
*
F
i
r
s
t
:
 
`
x
f
s
_
r
e
p
a
i
r
`
'
s
 
b
l
o
c
k
 
n
u
m
b
e
r
 
d
o
e
s
 
n
o
t
 
d
e
p
e
n
d
 
o
n
 
a
n
y
t
h
i
n
g
 
t
h
a
t
 
d
i
f
f
e
r
s


b
e
t
w
e
e
n
 
t
h
e
 
t
h
r
e
e
 
l
e
a
v
e
s
.
*
*
 
 
P
e
r
t
u
r
b
i
n
g
 
t
h
e
 
s
a
m
e
 
w
o
r
d
 
o
f
 
e
a
c
h
 
l
e
a
f
'
s
 
f
i
r
s
t
 
r
e
c
o
r
d
 
s
o


t
h
a
t
 
r
e
p
a
i
r
 
p
r
i
n
t
s
 
a
 
b
l
o
c
k
 
g
i
v
e
s
,
 
f
o
r
 
l
e
a
v
e
s
 
h
o
l
d
i
n
g
 
e
n
t
r
i
e
s
 
0
.
.
2
9
,
 
3
0
.
.
4
4
 
a
n
d


4
5
.
.
6
3
 
o
f
 
t
h
e
 
s
a
m
e
 
f
i
l
e
:




`
`
`
t
e
x
t


l
e
a
f
 
5
0
3
1
3
:
 
 
z
e
r
o
 
l
e
n
g
t
h
 
e
x
t
e
n
t
 
(
o
f
f
 
=
 
0
,
 
 
f
s
b
n
o
 
=
 
4
9
1
5
2
)


l
e
a
f
 
5
0
3
1
5
:
 
 
z
e
r
o
 
l
e
n
g
t
h
 
e
x
t
e
n
t
 
(
o
f
f
 
=
 
3
0
,
 
f
s
b
n
o
 
=
 
4
9
1
5
2
)


l
e
a
f
 
5
0
3
1
7
:
 
 
z
e
r
o
 
l
e
n
g
t
h
 
e
x
t
e
n
t
 
(
o
f
f
 
=
 
4
5
,
 
f
s
b
n
o
 
=
 
4
9
1
5
2
)


`
`
`




T
h
e
 
`
o
f
f
`
 
t
r
a
c
k
s
 
t
h
e
 
o
f
f
s
e
t
 
—
 
s
e
t
 
t
h
e
 
r
e
c
o
r
d
'
s
 
o
f
f
s
e
t
 
f
i
e
l
d
 
t
o
 
5
1
2
0
0
 
a
n
d
 
`
o
f
f
`


b
e
c
o
m
e
s
 
1
0
0
 
—
 
s
o
 
r
e
p
a
i
r
 
r
e
a
d
s
 
t
h
a
t
 
f
r
o
m
 
t
h
e
 
r
e
c
o
r
d
 
w
h
e
r
e
 
t
h
i
s
 
r
e
a
d
e
r
 
d
o
e
s
 
t
o
o
,
 
a
n
d


t
h
e
 
s
e
c
o
n
d
 
w
o
r
d
 
i
s
 
c
o
n
f
i
r
m
e
d
 
a
 
t
h
i
r
d
 
t
i
m
e
.
 
 
B
u
t
 
`
f
s
b
n
o
`
 
i
s
 
*
*
4
9
1
5
2
 
i
n
 
a
l
l
 
t
h
r
e
e
*
*
,


a
n
d
 
i
t
 
s
t
a
y
s
 
4
9
1
5
2
 
w
h
e
n
 
t
h
e
 
o
f
f
s
e
t
 
i
s
 
c
h
a
n
g
e
d
 
t
o
 
a
n
y
t
h
i
n
g
.
 
 
A
 
b
l
o
c
k
 
n
u
m
b
e
r
 
t
h
a
t
 
d
o
e
s


n
o
t
 
m
o
v
e
 
w
h
e
n
 
t
h
e
 
l
e
a
f
,
 
t
h
e
 
e
n
t
r
y
 
i
n
d
e
x
 
a
n
d
 
t
h
e
 
o
f
f
s
e
t
 
a
l
l
 
m
o
v
e
 
i
s
 
n
o
t
 
a
 
b
l
o
c
k
 
t
h
e


f
i
l
e
 
s
y
s
t
e
m
 
r
e
a
d
;
 
i
t
 
i
s
 
a
 
c
o
n
s
t
a
n
t
 
i
n
 
t
h
a
t
 
m
e
s
s
a
g
e
.




*
*
S
e
c
o
n
d
:
 
t
h
e
 
m
a
p
p
i
n
g
 
i
s
 
n
o
t
 
i
n
 
t
h
e
 
t
r
e
e
,
 
a
n
d
 
t
h
e
 
b
l
o
c
k
s
 
a
r
e
 
r
e
a
l
.
*
*
 
 
A
r
o
u
n
d
 
t
h
e


l
e
a
v
e
s
,
 
t
h
e
 
i
m
a
g
e
 
a
l
t
e
r
n
a
t
e
s
:




`
`
`
t
e
x
t


5
0
3
1
2
:
d
a
t
a
 
 
5
0
3
1
3
:
B
M
A
P
 
 
5
0
3
1
4
:
d
a
t
a
 
 
5
0
3
1
5
:
B
M
A
P
 
 
5
0
3
1
6
:
d
a
t
a
 
 
5
0
3
1
7
:
B
M
A
P
 
 
.
.
.


`
`
`




a
n
d
 
t
h
e
 
d
a
t
a
 
b
l
o
c
k
s
 
h
o
l
d
 
t
h
e
 
f
i
l
e
'
s
 
c
o
n
t
e
n
t
 
—
 
a
s
c
e
n
d
i
n
g
 
c
o
u
n
t
e
r
s
 
a
s
 
t
e
x
t
,
 
a
t
 
5
0
3
1
2
,


5
0
3
1
4
,
 
5
0
3
1
6
 
a
n
d
 
5
0
3
1
8
,
 
w
h
i
c
h
 
a
r
e
 
e
x
a
c
t
l
y
 
t
h
e
 
b
l
o
c
k
s
 
`
x
f
s
_
r
e
p
a
i
r
`
 
p
r
i
n
t
e
d
 
f
o
r


e
n
t
r
i
e
s
 
0
,
 
1
,
 
2
 
a
n
d
 
3
.
 
 
S
o
 
t
h
e
 
e
x
t
e
n
t
s
 
r
e
a
l
l
y
 
a
r
e
 
a
t
 
`
5
0
3
1
2
 
+
 
2
n
`
,
 
a
n
d
 
t
h
e
 
t
r
e
e


r
e
a
l
l
y
 
d
o
e
s
 
n
o
t
 
s
a
y
 
s
o
:
 
t
h
e
 
f
i
l
e
'
s
 
i
n
o
d
e
 
h
o
l
d
s
 
a
 
f
o
r
k
 
w
i
t
h
 
t
h
r
e
e
 
k
e
y
s
 
a
n
d
 
t
h
r
e
e


p
o
i
n
t
e
r
s
,
 
e
a
c
h
 
p
o
i
n
t
e
r
 
n
a
m
e
s
 
a
 
l
e
a
f
,
 
a
n
d
 
e
a
c
h
 
l
e
a
f
 
h
o
l
d
s
 
o
f
f
s
e
t
s
 
a
n
d
 
l
e
n
g
t
h
s
.
 
 
T
h
e


b
l
o
c
k
s
 
a
r
e
 
n
o
w
h
e
r
e
 
i
n
 
i
t
.




*
*
S
o
 
`
x
f
s
v
4
.
i
m
g
`
'
s
 
b
-
t
r
e
e
 
f
i
l
e
s
 
h
a
v
e
 
n
o
 
e
x
t
e
n
t
-
t
o
-
b
l
o
c
k
 
m
a
p
p
i
n
g
 
o
n
 
d
i
s
k
,
 
a
n
d


`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
c
o
n
s
t
r
u
c
t
s
 
o
n
e
.
*
*
 
 
I
t
 
c
a
n
,
 
h
e
r
e
,
 
o
n
l
y
 
b
e
c
a
u
s
e
 
t
h
e
 
b
u
i
l
d
e
r
 
a
l
l
o
c
a
t
e
d
 
a


d
a
t
a
 
b
l
o
c
k
 
a
n
d
 
a
 
t
r
e
e
 
n
o
d
e
 
a
l
t
e
r
n
a
t
e
l
y
,
 
w
h
i
c
h
 
m
a
k
e
s
 
t
h
e
 
m
a
p
p
i
n
g
 
d
e
r
i
v
a
b
l
e
 
f
r
o
m
 
t
h
e


e
n
t
r
y
 
i
n
d
e
x
 
—
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
`
'
s
 
`
s
`
 
v
a
l
u
e
s
 
f
o
l
l
o
w
 
t
h
e
 
e
n
t
r
y
 
i
n
d
e
x
 
p
r
e
c
i
s
e
l
y


b
e
c
a
u
s
e
 
t
h
a
t
 
i
s
 
w
h
a
t
 
i
t
 
i
s
 
d
o
i
n
g
.




*
*
W
h
i
c
h
 
m
e
a
n
s
 
t
h
e
 
b
l
o
c
k
 
f
i
e
l
d
'
s
 
p
o
s
i
t
i
o
n
 
c
a
n
n
o
t
 
b
e
 
m
e
a
s
u
r
e
d
 
f
r
o
m
 
t
h
i
s
 
r
e
p
o
s
i
t
o
r
y
'
s


i
m
a
g
e
s
*
*
,
 
f
o
r
 
t
h
e
 
s
a
m
e
 
r
e
a
s
o
n
 
t
h
e
 
A
G
F
L
→
f
r
e
e
-
s
p
a
c
e
 
q
u
e
s
t
i
o
n
 
c
o
u
l
d
 
n
o
t
 
b
e
:
 
t
h
e


s
u
b
s
t
r
a
t
e
 
d
o
e
s
 
n
o
t
 
c
o
n
t
a
i
n
 
t
h
e
 
s
t
r
u
c
t
u
r
e
.
 
 
N
o
t
 
"
w
e
 
h
a
v
e
 
n
o
t
 
f
o
u
n
d
 
i
t
 
y
e
t
"
 
—
 
t
h
e


s
t
r
u
c
t
u
r
e
 
i
s
 
n
o
t
 
t
h
e
r
e
 
t
o
 
f
i
n
d
.
 
 
`
s
c
r
i
p
t
s
/
m
k
i
m
g
.
x
f
s
`
 
b
u
i
l
t
 
t
h
e
s
e
 
f
i
l
e
s
 
w
i
t
h


s
o
m
e
t
h
i
n
g
 
o
t
h
e
r
 
t
h
a
n
 
a
 
b
-
m
a
p
 
w
r
i
t
e
r
,
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
`
 
i
s
 
l
e
n
i
e
n
t
 
a
b
o
u
t
 
i
t
 
b
e
c
a
u
s
e


i
t
 
c
a
n
 
p
a
p
e
r
 
o
v
e
r
 
t
h
e
 
r
e
s
u
l
t
.




W
h
a
t
 
t
h
a
t
 
c
o
s
t
s
,
 
s
t
a
t
e
d
 
p
l
a
i
n
l
y
:




*
 
t
h
e
 
b
-
m
a
p
 
*
*
r
e
a
d
e
r
*
*
 
h
e
r
e
 
c
a
n
n
o
t
 
b
e
 
f
i
n
i
s
h
e
d
,
 
a
n
d
 
t
h
e
 
o
n
e
 
t
h
a
t
 
e
x
i
s
t
s
 
i
s


 
 
p
r
o
v
i
s
i
o
n
a
l
 
b
y
 
c
o
n
s
t
r
u
c
t
i
o
n
 
r
a
t
h
e
r
 
t
h
a
n
 
b
y
 
c
a
u
t
i
o
n
 
—
 
t
h
e
r
e
 
i
s
 
n
o
t
h
i
n
g
 
i
n
 
t
h
e
s
e


 
 
i
m
a
g
e
s
 
t
o
 
r
e
a
d
;


*
 
i
n
-
p
l
a
c
e
 
o
v
e
r
w
r
i
t
e
 
o
f
 
a
 
b
-
t
r
e
e
-
b
a
c
k
e
d
 
f
i
l
e
 
i
s
 
*
*
r
e
f
u
s
e
d
*
*
,
 
a
n
d
 
t
h
e
 
t
e
s
t
 
t
h
a
t
 
u
s
e
d


 
 
t
o
 
w
r
i
t
e
 
i
n
t
o
 
t
h
r
e
e
 
s
u
c
h
 
f
i
l
e
s
 
n
o
w
 
s
k
i
p
s
 
t
h
e
m
 
w
i
t
h
 
t
h
e
 
r
e
a
s
o
n
 
p
r
i
n
t
e
d
;


*
 
a
n
 
i
m
a
g
e
 
w
i
t
h
 
a
 
*
r
e
a
l
*
 
b
-
m
a
p
 
f
i
l
e
,
 
b
u
i
l
t
 
b
y
 
`
m
k
f
s
.
x
f
s
`
,
 
w
o
u
l
d
 
s
e
t
t
l
e
 
t
h
e
 
b
l
o
c
k


 
 
f
i
e
l
d
 
i
n
 
o
n
e
 
p
e
r
t
u
r
b
a
t
i
o
n
 
o
f
 
o
n
e
 
a
c
c
e
p
t
e
d
 
r
e
c
o
r
d
 
—
 
a
n
d
 
n
o
n
e
 
o
f
 
t
h
e
 
t
h
r
e
e
 
i
m
a
g
e
s


 
 
h
e
r
e
 
h
a
s
 
o
n
e
,
 
s
o
 
t
h
a
t
 
e
x
p
e
r
i
m
e
n
t
 
n
e
e
d
s
 
a
 
s
u
b
s
t
r
a
t
e
 
t
h
i
s
 
e
n
v
i
r
o
n
m
e
n
t
 
c
a
n
n
o
t
 
m
a
k
e
.




#
#
#
 
T
h
e
 
a
t
t
e
m
p
t
 
t
h
a
t
 
f
a
i
l
e
d
,
 
a
n
d
 
w
h
y
 
i
t
 
i
s
 
w
o
r
t
h
 
w
r
i
t
i
n
g
 
d
o
w
n






a
t
t
e
m
p
t
 
t
o
 
s
e
t
t
l
e
 
i
t
 
b
y
 
h
a
n
d
 
i
s
 
w
o
r
t
h
 
r
e
c
o
r
d
i
n
g
 
a
s
 
a
 
f
a
i
l
u
r
e
 
o
f
 
m
e
t
h
o
d
 
r
a
t
h
e
r
 
t
h
a
n


o
f
 
a
r
i
t
h
m
e
t
i
c
.
 
 
P
a
t
c
h
i
n
g
 
a
 
s
l
o
t
 
a
n
d
 
a
s
k
i
n
g
 
r
e
p
a
i
r
 
p
r
o
d
u
c
e
s
 
a
 
c
o
m
p
l
a
i
n
t
 
w
h
o
s
e


n
u
m
b
e
r
s
 
c
a
n
 
b
e
 
r
e
a
d
 
s
e
v
e
r
a
l
 
w
a
y
s
,
 
a
n
d
 
f
o
u
r
 
c
a
n
d
i
d
a
t
e
 
l
a
y
o
u
t
s
 
a
l
l
 
p
r
o
d
u
c
e
d


c
o
m
p
l
a
i
n
t
s
,
 
n
o
n
e
 
o
f
 
t
h
e
m
 
c
l
e
a
n
 
-
-
 
b
e
c
a
u
s
e
 
t
h
e
 
e
x
p
e
r
i
m
e
n
t
 
w
a
s
 
w
r
o
n
g
,
 
n
o
t
 
t
h
e


c
a
n
d
i
d
a
t
e
s
:
 
i
t
 
r
e
w
r
o
t
e
 
o
n
e
 
l
e
a
f
 
o
f
 
a
 
t
h
r
e
e
-
l
e
a
f
 
t
r
e
e
,
 
l
e
a
v
i
n
g
 
t
h
e
 
i
n
t
e
r
i
o
r
 
n
o
d
e


c
l
a
i
m
i
n
g
 
t
h
r
e
e
 
c
h
i
l
d
r
e
n
,
 
s
o
 
r
e
p
a
i
r
 
w
a
l
k
e
d
 
t
h
e
 
u
n
t
o
u
c
h
e
d
 
l
e
a
v
e
s
 
t
o
o
 
a
n
d
 
e
v
e
r
y


c
o
m
p
l
a
i
n
t
 
w
a
s
 
a
b
o
u
t
 
s
o
m
e
t
h
i
n
g
 
e
l
s
e
.
 
 
*
*
T
h
e
 
s
e
t
u
p
 
h
a
s
 
t
o
 
b
e
 
c
o
h
e
r
e
n
t
 
b
e
f
o
r
e
 
t
h
e


q
u
e
s
t
i
o
n
 
i
s
 
a
s
k
a
b
l
e
*
*
,
 
a
n
d
 
t
h
e
 
r
e
c
i
p
e
 
i
s
 
i
n
 
t
h
e
 
s
e
c
t
i
o
n
 
a
b
o
v
e
:
 
a
 
s
i
n
g
l
e
 
l
e
a
f
,
 
a


r
e
c
o
r
d
 
w
i
t
h
 
a
n
 
u
n
a
m
b
i
g
u
o
u
s
 
s
t
a
r
t
o
f
f
,
 
t
h
e
 
b
l
o
c
k
 
n
u
m
b
e
r
 
i
n
 
o
n
e
 
c
a
n
d
i
d
a
t
e
 
f
i
e
l
d
 
o
r
 
t
h
e


o
t
h
e
r
,
 
a
n
d
 
a
 
f
o
r
k
 
h
e
a
d
e
r
 
t
h
a
t
 
n
a
m
e
s
 
t
h
a
t
 
o
n
e
 
l
e
a
f
 
a
t
 
l
e
v
e
l
 
0
.
 
 
T
h
e
n
 
t
h
e
 
o
n
l
y


c
o
m
p
l
a
i
n
t
 
r
e
p
a
i
r
 
c
a
n
 
p
o
s
s
i
b
l
y
 
m
a
k
e
 
i
s
 
a
b
o
u
t
 
t
h
e
 
f
i
e
l
d
 
t
h
a
t
 
i
s
 
w
r
o
n
g
,
 
a
n
d
 
w
h
e
r
e
 
i
t


n
a
m
e
s
 
a
n
 
o
f
f
s
e
t
 
*
i
s
*
 
t
h
e
 
l
a
y
o
u
t
.




A
l
s
o
 
w
o
r
t
h
 
k
n
o
w
i
n
g
 
b
e
f
o
r
e
 
a
n
y
o
n
e
 
t
r
i
e
s
:
 
t
h
e
 
b
-
t
r
e
e
 
p
a
t
h
 
i
n
 
t
h
i
s
 
c
o
d
e
 
i
s
 
n
o
t
 
o
n
l
y
 
a


*
w
r
i
t
e
r
*
.
 
 
`
B
t
r
e
e
B
l
o
c
k
H
d
r
`
 
h
a
s
 
n
o
 
n
o
t
i
o
n
 
o
f
 
a
 
b
-
m
a
p
 
b
l
o
c
k
,
 
`
D
i
U
:
:
B
m
b
t
`
 
d
e
s
c
r
i
b
e
s
 
a


f
o
r
k
 
h
e
a
d
e
r
 
r
a
t
h
e
r
 
t
h
a
n
 
a
 
n
o
d
e
'
s
 
r
e
c
o
r
d
s
,
 
a
n
d
 
`
E
x
t
e
n
t
M
a
p
`
'
s
 
b
-
t
r
e
e
 
a
r
m
 
c
a
n
n
o
t
 
f
e
t
c
h


a
 
l
e
a
f
.
 
 
S
o
 
a
 
s
p
i
l
l
 
i
s
 
r
e
a
d
e
r
-
t
h
e
n
-
w
r
i
t
e
r
,
 
i
n
 
t
h
a
t
 
o
r
d
e
r
,
 
a
n
d
 
t
h
e
 
r
e
a
d
e
r
 
i
s
 
a


m
i
l
e
s
t
o
n
e
 
o
f
 
i
t
s
 
o
w
n
 
r
a
t
h
e
r
 
t
h
a
n
 
a
 
d
e
t
a
i
l
 
o
f
 
t
h
e
 
w
r
i
t
e
r
.




#
#
#
 
E
v
e
r
y
 
d
i
r
e
c
t
o
r
y
 
i
n
 
t
h
e
s
e
 
i
m
a
g
e
s
 
i
s
 
i
n
 
*
l
o
c
a
l
*
 
f
o
r
m
a
t




`
x
f
s
_
d
b
`
 
o
n
 
`
x
f
s
v
4
.
i
m
g
`
 
r
e
p
o
r
t
s
 
`
/
f
i
l
e
s
`
 
w
i
t
h
 
`
c
o
r
e
.
f
o
r
m
a
t
 
=
 
1
 
(
l
o
c
a
l
)
`
,
 
a
n
d
 
t
h
e


s
a
m
e
 
f
o
r
 
t
h
e
 
r
o
o
t
:
 
t
h
e
i
r
 
e
n
t
r
i
e
s
 
a
r
e
 
p
a
c
k
e
d
 
i
n
t
o
 
t
h
e
 
i
n
o
d
e
'
s
 
d
a
t
a
 
a
r
e
a
 
r
a
t
h
e
r
 
t
h
a
n


i
n
t
o
 
b
l
o
c
k
s
.
 
 
S
o
 
`
c
r
e
a
t
e
`
 
h
e
r
e
 
w
o
u
l
d
 
b
e
 
i
m
p
l
e
m
e
n
t
i
n
g
 
i
n
s
e
r
t
i
o
n
 
i
n
t
o
 
a


*
l
o
c
a
l
-
f
o
r
m
a
t
*
 
d
i
r
e
c
t
o
r
y
 
—
 
h
a
s
h
e
d
 
e
n
t
r
y
 
o
r
d
e
r
,
 
n
o
t
 
t
h
e
 
f
l
a
t
 
b
l
o
c
k
 
l
a
y
o
u
t
 
t
h
a
t
 
w
o
u
l
d


b
e
 
e
a
s
i
e
r
 
—
 
a
n
d
 
n
o
n
e
 
o
f
 
t
h
e
 
i
m
a
g
e
s
 
e
x
e
r
c
i
s
e
s
 
t
h
e
 
b
l
o
c
k
-
d
i
r
e
c
t
o
r
y
 
c
a
s
e
 
a
t
 
a
l
l
.


T
h
a
t
 
i
s
 
a
 
f
i
n
d
i
n
g
 
a
b
o
u
t
 
t
h
e
 
s
u
b
s
t
r
a
t
e
 
r
a
t
h
e
r
 
t
h
a
n
 
a
b
o
u
t
 
t
h
e
 
c
o
d
e
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e


r
e
a
s
o
n
 
`
c
r
e
a
t
e
`
 
h
a
s
 
n
o
t
 
b
e
e
n
 
b
u
i
l
t
 
o
n
 
t
o
p
 
o
f
 
a
 
g
u
e
s
s
 
a
b
o
u
t
 
w
h
a
t
 
t
h
e
s
e
 
d
i
r
e
c
t
o
r
i
e
s


l
o
o
k
 
l
i
k
e
.




#
#
#
 
W
h
a
t
 
t
h
e
 
s
u
b
t
r
e
e
 
t
h
e
r
e
f
o
r
e
 
n
e
e
d
s




1
.
 
A
 
c
h
u
n
k
 
s
t
a
r
t
 
s
e
a
r
c
h
e
d
 
f
o
r
,
 
n
o
t
 
c
o
m
p
u
t
e
d
:
 
c
a
n
d
i
d
a
t
e
 
i
n
o
d
e
 
n
u
m
b
e
r
s
 
a
t
 
n
o
m
i
n
a
l


 
 
 
c
h
u
n
k
 
b
o
u
n
d
a
r
i
e
s
,
 
r
e
j
e
c
t
e
d
 
u
n
l
e
s
s
 
a
l
l
 
3
2
 
o
f
 
t
h
e
 
c
h
u
n
k
'
s
 
b
l
o
c
k
s
 
a
r
e
 
f
r
e
e
 
i
n
 
t
h
e


 
 
 
b
n
o
 
t
r
e
e
 
a
n
d
 
a
r
e
 
n
o
t
 
t
h
e
 
g
r
o
u
p
'
s
 
o
w
n
 
m
e
t
a
d
a
t
a
.


2
.
 
3
2
 
b
l
o
c
k
s
 
a
l
l
o
c
a
t
e
d
 
f
r
o
m
 
t
h
e
 
g
r
o
u
p
'
s
 
f
r
e
e
 
s
p
a
c
e
,
 
t
h
r
o
u
g
h
 
t
h
e
 
o
r
d
i
n
a
r
y


 
 
 
t
r
a
n
s
a
c
t
i
o
n
,
 
s
o
 
t
h
e
 
g
r
o
u
p
'
s
 
c
o
u
n
t
s
 
f
o
l
l
o
w
.


3
.
 
6
4
 
s
l
o
t
s
 
w
r
i
t
t
e
n
 
i
n
 
t
h
e
 
l
a
y
o
u
t
 
a
b
o
v
e
 
—
 
w
h
i
c
h
 
i
s
 
a
 
c
h
o
i
c
e
,
 
n
o
t
 
a
 
m
e
a
s
u
r
e
m
e
n
t
,


 
 
 
a
n
d
 
t
h
e
 
d
o
c
u
m
e
n
t
a
t
i
o
n
 
s
a
y
s
 
s
o
 
a
t
 
t
h
e
 
p
l
a
c
e
 
i
t
 
i
s
 
m
a
d
e
.


4
.
 
A
n
 
I
N
O
B
T
 
r
e
c
o
r
d
 
w
i
t
h
 
`
s
t
a
r
t
i
n
o
`
,
 
`
f
r
e
e
c
o
u
n
t
 
=
 
6
4
`
 
a
n
d
 
a
l
l
 
6
4
 
m
a
s
k
 
b
i
t
s
 
s
e
t
,


 
 
 
i
n
s
e
r
t
e
d
 
i
n
 
k
e
y
 
o
r
d
e
r
,
 
w
i
t
h
 
`
f
r
e
e
c
o
u
n
t
 
=
=
 
p
o
p
c
o
u
n
t
(
f
r
e
e
_
m
a
s
k
)
`
 
c
h
e
c
k
e
d
 
b
e
f
o
r
e


 
 
 
a
n
d
 
a
f
t
e
r
 
a
s
 
t
h
e
 
p
l
a
n
 
r
e
q
u
i
r
e
s
.


5
.
 
T
h
e
 
g
r
o
u
p
'
s
 
i
n
o
d
e
 
h
e
a
d
e
r
:
 
`
c
o
u
n
t
 
+
=
 
6
4
`
,
 
`
f
r
e
e
c
o
u
n
t
 
+
=
 
6
4
`
,
 
`
i
n
o
_
b
l
o
c
k
s
 
+
=
 
3
2
`
,


 
 
 
a
n
d
 
`
n
e
w
i
n
o
`
 
s
e
t
 
t
o
 
t
h
e
 
n
e
w
 
c
h
u
n
k
'
s
 
s
t
a
r
t
 
—
 
t
h
e
 
o
n
e
 
p
l
a
c
e
 
`
n
e
w
i
n
o
`
 
m
o
v
e
s
,
 
w
h
i
c
h


 
 
 
i
s
 
w
h
a
t
 
m
a
k
e
s
 
i
t
 
a
 
h
i
n
t
 
a
b
o
u
t
 
*
c
h
u
n
k
s
*
 
r
a
t
h
e
r
 
t
h
a
n
 
a
b
o
u
t
 
t
h
e
 
l
a
s
t
 
i
n
o
d
e
 
h
a
n
d
e
d


 
 
 
o
u
t
.


6
.
 
`
s
b
_
i
f
r
e
e
 
+
=
 
6
4
`
.


7
.
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
a
s
 
t
h
e
 
j
u
d
g
e
,
 
b
e
c
a
u
s
e
 
e
v
e
r
y
t
h
i
n
g
 
a
b
o
v
e
 
i
s
 
a
 
c
l
a
i
m
 
a
b
o
u
t
 
a
 
f
i
l
e


 
 
 
s
y
s
t
e
m
 
a
n
d
 
n
o
t
h
i
n
g
 
e
l
s
e
 
h
e
r
e
 
c
a
n
 
c
h
e
c
k
 
o
n
e
.




T
h
e
 
s
u
b
s
t
r
a
t
e
 
e
x
i
s
t
s
 
a
n
d
 
i
s
 
r
e
p
a
i
r
-
c
l
e
a
n
:
 
`
x
f
s
_
w
r
i
t
a
b
l
e
.
i
m
g
`
'
s
 
g
r
o
u
p
s
 
1
,
 
2
 
a
n
d
 
3


h
a
v
e
 
a
n
 
*
*
e
m
p
t
y
*
*
 
i
n
o
d
e
 
t
r
e
e
 
(
`
c
o
u
n
t
 
=
 
0
`
,
 
`
f
r
e
e
c
o
u
n
t
 
=
 
0
`
,
 
o
n
e
 
l
e
a
f
 
w
i
t
h
 
n
o


r
e
c
o
r
d
s
)
,
 
s
o
 
a
 
f
i
r
s
t
 
c
h
u
n
k
 
i
n
 
a
 
g
r
o
u
p
 
i
s
 
a
 
c
a
s
e
 
w
i
t
h
 
n
o
 
o
v
e
r
l
a
p
 
h
a
z
a
r
d
s
 
i
n
 
i
t
.




#
#
#
 
L
e
t
t
i
n
g
 
t
h
e
 
t
r
e
e
 
g
r
o
w
,
 
a
n
d
 
w
h
a
t
 
t
h
a
t
 
c
o
s
t




T
h
e
 
t
r
e
e
 
h
a
d
 
t
o
 
g
r
o
w
 
b
e
f
o
r
e
 
a
n
y
 
o
f
 
t
h
e
 
a
b
o
v
e
 
w
a
s
 
w
o
r
t
h
 
m
u
c
h
,
 
a
n
d
 
i
t
 
w
a
s
 
n
o
t
 
a


c
o
r
n
e
r
 
c
a
s
e
:
 
a
 
l
e
a
f
 
i
n
 
a
 
5
1
2
-
b
y
t
e
 
b
l
o
c
k
 
h
o
l
d
s
 
3
1
 
r
e
c
o
r
d
s
 
a
n
d
 
`
x
f
s
v
4
.
i
m
g
`
'
s
 
g
r
o
u
p
 
1


h
a
s
 
*
*
s
e
v
e
n
 
o
f
 
i
t
s
 
n
i
n
e
 
l
e
a
v
e
s
 
a
l
r
e
a
d
y
 
f
u
l
l
*
*
,
 
s
o
 
t
h
e
 
e
i
g
h
t
h
 
c
h
u
n
k
 
i
n
s
e
r
t
e
d
 
t
h
e
r
e


i
s
 
t
h
e
 
o
n
e
 
t
h
a
t
 
s
p
l
i
t
s
.
 
 
`
i
n
s
e
r
t
_
c
h
u
n
k
`
 
w
a
l
k
s
 
t
o
 
t
h
e
 
l
e
a
f
,
 
s
p
l
i
t
s
 
i
t
 
i
n
 
h
a
l
f
,
 
l
i
n
k
s


t
h
e
 
n
e
w
 
n
o
d
e
 
i
n
t
o
 
t
h
e
 
s
i
b
l
i
n
g
 
c
h
a
i
n
 
*
*
o
n
 
b
o
t
h
 
s
i
d
e
s
*
*
,
 
g
i
v
e
s
 
t
h
e
 
p
a
r
e
n
t
 
a
 
n
e
w


c
h
i
l
d
,
 
r
e
c
u
r
s
e
s
 
w
h
e
n
 
t
h
e
 
p
a
r
e
n
t
 
h
a
s
 
n
o
 
r
o
o
m
,
 
a
n
d
 
g
r
o
w
s
 
a
 
n
e
w
 
r
o
o
t
 
w
h
e
n
 
t
h
e
 
t
r
e
e
 
w
a
s


a
 
s
i
n
g
l
e
 
l
e
a
f
.




T
h
r
e
e
 
b
u
g
s
 
i
t
 
t
o
o
k
,
 
a
n
d
 
e
a
c
h
 
i
s
 
a
 
t
h
i
n
g
 
o
n
l
y
 
a
 
w
a
l
k
 
o
r
 
t
h
e
 
o
r
a
c
l
e
 
c
o
u
l
d
 
s
e
e
:




*
 
*
*
T
h
e
 
s
u
c
c
e
s
s
f
u
l
 
i
n
s
e
r
t
 
p
a
t
h
 
n
e
v
e
r
 
w
r
o
t
e
 
t
h
e
 
l
e
a
f
 
b
a
c
k
.
*
*
 
 
T
h
e
 
i
n
s
e
r
t
 
s
u
c
c
e
e
d
e
d
,


 
 
t
h
e
 
t
r
e
e
 
w
a
s
 
u
n
c
h
a
n
g
e
d
,
 
t
h
e
 
c
a
l
l
e
r
 
b
e
l
i
e
v
e
d
 
t
h
e
 
c
h
u
n
k
 
w
a
s
 
t
h
e
r
e
,
 
a
n
d
 
n
o
t
h
i
n
g


 
 
a
n
y
w
h
e
r
e
 
r
e
p
o
r
t
e
d
 
a
n
 
e
r
r
o
r
.
 
 
T
h
e
 
t
r
e
e
 
a
l
s
o
 
n
e
v
e
r
 
g
r
e
w
,
 
b
e
c
a
u
s
e
 
t
h
e
 
l
e
a
f
 
i
t
 
k
e
p
t


 
 
l
a
n
d
i
n
g
 
i
n
 
w
a
s
 
n
e
v
e
r
 
a
n
y
 
f
u
l
l
e
r
 
—
 
f
o
u
n
d
 
b
y
 
a
 
t
e
s
t
 
t
h
a
t
 
a
d
d
e
d
 
t
w
o
 
h
u
n
d
r
e
d
 
c
h
u
n
k
s
 
t
o


 
 
a
 
g
r
o
u
p
 
w
h
o
s
e
 
l
e
a
v
e
s
 
w
e
r
e
 
f
u
l
l
 
a
n
d
 
w
a
t
c
h
e
d
 
n
o
n
e
 
o
f
 
t
h
e
m
 
s
p
l
i
t
.




*
 
*
*
T
h
e
 
s
p
l
i
t
 
d
r
o
p
p
e
d
 
t
h
e
 
r
e
c
o
r
d
 
i
t
 
w
a
s
 
a
s
k
e
d
 
t
o
 
i
n
s
e
r
t
.
*
*
 
 
A
 
s
p
l
i
t
 
m
a
k
e
s
 
r
o
o
m
;
 
i
t


 
 
d
o
e
s
 
n
o
t
 
u
s
e
 
i
t
.
 
 
T
h
e
 
c
h
u
n
k
 
w
e
n
t
 
o
n
 
t
h
e
 
f
l
o
o
r
,
 
w
h
i
c
h
 
s
h
o
w
s
 
u
p
 
a
 
w
h
o
l
e
 
c
h
u
n
k
 
s
h
o
r
t


 
 
i
n
 
t
w
o
 
c
o
u
n
t
e
r
s
:




 
 
`
`
`
t
e
x
t


 
 
a
g
i
_
c
o
u
n
t
 
1
6
9
6
0
,
 
c
o
u
n
t
e
d
 
1
6
8
9
6
 
i
n
 
a
g
 
1


 
 
s
b
_
i
c
o
u
n
t
 
2
2
6
5
6
,
 
c
o
u
n
t
e
d
 
2
2
5
9
2


 
 
`
`
`




*
 
*
*
A
n
 
i
n
o
d
e
-
t
r
e
e
 
n
o
d
e
 
i
s
 
c
h
a
r
g
e
d
 
d
i
f
f
e
r
e
n
t
l
y
 
f
r
o
m
 
a
 
f
r
e
e
-
s
p
a
c
e
 
n
o
d
e
*
*
,
 
a
n
d
 
n
e
i
t
h
e
r


 
 
w
a
y
 
i
s
 
w
h
a
t
 
t
a
k
i
n
g
 
o
n
e
 
a
s
s
u
m
e
d
:




 
 
`
`
`
t
e
x
t


 
 
a
g
f
_
b
t
r
e
e
b
l
k
s
 
5
9
,
 
c
o
u
n
t
e
d
 
5
8
 
i
n
 
a
g
 
1


 
 
s
b
_
f
d
b
l
o
c
k
s
 
9
0
3
6
8
,
 
c
o
u
n
t
e
d
 
9
0
3
6
7


 
 
`
`
`




 
 
`
a
g
f
_
b
t
r
e
e
b
l
k
s
`
 
c
o
u
n
t
s
 
t
h
e
 
b
l
o
c
k
s
 
t
h
e
 
t
w
o
 
*
*
f
r
e
e
 
s
p
a
c
e
*
*
 
t
r
e
e
s
 
h
o
l
d
,
 
a
n
d
 
a
 
n
o
d
e
 
o
f


 
 
t
h
e
 
i
n
o
d
e
 
t
r
e
e
 
i
s
 
n
o
t
 
o
n
e
 
o
f
 
t
h
o
s
e
,
 
s
o
 
c
h
a
r
g
i
n
g
 
i
t
 
c
h
a
r
g
e
s
 
t
h
e
 
g
r
o
u
p
 
f
o
r
 
a
 
t
r
e
e
 
i
t


 
 
d
o
e
s
 
n
o
t
 
h
a
v
e
.
 
 
A
n
d
 
a
 
b
l
o
c
k
 
t
h
a
t
 
h
a
s
 
b
e
c
o
m
e
 
a
n
 
i
n
o
d
e
 
t
r
e
e
 
n
o
d
e
 
i
s
 
i
n
 
n
o
n
e
 
o
f
 
t
h
e


 
 
t
h
r
e
e
 
p
l
a
c
e
s
 
a
 
f
r
e
e
 
b
l
o
c
k
 
i
s
 
a
c
c
o
u
n
t
e
d
 
f
o
r
,
 
s
o
 
t
h
e
 
d
e
v
i
c
e
'
s
 
f
r
e
e
 
c
o
u
n
t
 
f
a
l
l
s
 
b
y
 
o
n
e


 
 
—
 
u
n
l
i
k
e
 
a
 
f
r
e
e
-
s
p
a
c
e
 
n
o
d
e
,
 
w
h
i
c
h
 
t
r
a
d
e
s
 
o
n
e
 
t
e
r
m
 
f
o
r
 
a
n
o
t
h
e
r
 
a
n
d
 
l
e
a
v
e
s
 
t
h
e
 
t
o
t
a
l


 
 
a
l
o
n
e
.
 
 
T
h
a
t
 
i
s
 
a
 
f
o
u
r
t
h
 
t
e
r
m
 
t
h
e
 
[
t
h
r
e
e
-
t
e
r
m
 
i
d
e
n
t
i
t
y
]
(
#
t
h
e
-
s
u
p
e
r
b
l
o
c
k
s
-
f
r
e
e
-
c
o
u
n
t
-
h
a
s
-
t
h
r
e
e
-
t
e
r
m
s
-
a
n
d
-
a
l
l
-
t
h
r
e
e
-
a
r
e
-
m
e
a
s
u
r
e
d
)


 
 
d
i
d
 
n
o
t
 
h
a
v
e
,
 
a
n
d
 
i
t
 
i
s
 
t
h
e
r
e
 
b
e
c
a
u
s
e
 
a
n
 
i
n
o
d
e
 
t
r
e
e
 
n
o
d
e
 
i
s
 
c
o
u
n
t
e
d
 
n
o
w
h
e
r
e
.




A
n
d
 
o
n
e
 
b
u
g
 
i
n
 
t
h
e
 
*
c
h
e
c
k
*
 
r
a
t
h
e
r
 
t
h
a
n
 
i
n
 
t
h
e
 
c
o
d
e
,
 
w
h
i
c
h
 
i
s
 
w
o
r
t
h
 
r
e
c
o
r
d
i
n
g


b
e
c
a
u
s
e
 
i
t
 
m
a
k
e
s
 
a
 
r
e
a
l
 
f
a
u
l
t
 
l
o
o
k
 
l
i
k
e
 
a
n
 
i
m
a
g
i
n
a
r
y
 
o
n
e
:
 
t
h
e
 
s
i
b
l
i
n
g
-
c
h
a
i
n
 
c
h
e
c
k


c
o
m
p
a
r
e
d
 
t
h
e
 
c
h
a
i
n
 
a
g
a
i
n
s
t
 
a
 
l
i
s
t
 
o
f
 
l
e
a
f
 
b
l
o
c
k
s
 
*
*
s
o
r
t
e
d
 
b
y
 
b
l
o
c
k
 
n
u
m
b
e
r
*
*
,
 
w
h
i
c
h
 
i
s


n
o
t
 
t
h
e
 
c
h
a
i
n
'
s
 
o
r
d
e
r
,
 
a
n
d
 
s
o
 
r
e
p
o
r
t
e
d
 
t
h
e
 
p
r
i
s
t
i
n
e
 
u
n
t
o
u
c
h
e
d
 
i
m
a
g
e
'
s
 
o
w
n
 
c
h
a
i
n
 
a
s


b
r
o
k
e
n
.
 
 
T
h
e
 
c
h
a
i
n
 
n
o
w
 
g
i
v
e
s
 
t
h
e
 
o
r
d
e
r
 
—
 
s
t
a
r
t
 
a
t
 
t
h
e
 
l
e
a
f
 
w
i
t
h
 
n
o
 
l
e
f
t
 
n
e
i
g
h
b
o
u
r
 
a
n
d


w
a
l
k
 
r
i
g
h
t
 
—
 
a
n
d
 
b
o
t
h
 
s
i
d
e
s
 
a
r
e
 
c
h
e
c
k
e
d
 
a
t
 
e
v
e
r
y
 
s
t
e
p
.




R
o
o
t
 
c
o
l
l
a
p
s
e
 
i
s
 
s
t
i
l
l
 
n
o
t
 
w
r
i
t
t
e
n
,
 
a
n
d
 
t
h
a
t
 
i
s
 
n
o
t
 
a
n
 
o
v
e
r
s
i
g
h
t
:
 
n
o
t
h
i
n
g
 
e
m
p
t
i
e
s
 
a


p
a
r
e
n
t
,
 
b
e
c
a
u
s
e
 
a
 
p
a
r
e
n
t
 
a
l
w
a
y
s
 
h
o
l
d
s
 
a
t
 
l
e
a
s
t
 
t
w
o
 
c
h
i
l
d
r
e
n
 
a
n
d
 
t
h
e
 
m
e
r
g
e
 
b
r
a
n
c
h


r
e
f
u
s
e
s
 
t
o
 
t
a
k
e
 
t
h
e
 
l
a
s
t
 
o
n
e
.
 
 
I
t
 
w
i
l
l
 
b
e
 
w
r
i
t
t
e
n
 
w
h
e
n
 
a
 
t
e
s
t
 
r
e
a
c
h
e
s
 
i
t
.




#
#
 
T
h
e
 
A
G
F
L
 
→
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
 
q
u
e
s
t
i
o
n
,
 
a
n
s
w
e
r
e
d




T
h
e
 
p
l
a
n
 
a
s
k
s
 
w
h
a
t
 
t
h
e
 
"
A
G
F
L
 
→
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
"
 
t
r
a
n
s
i
t
i
o
n
 
i
s
:
 
w
h
a
t
 
h
a
p
p
e
n
s


w
h
e
n
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
 
i
s
 
r
e
l
e
a
s
e
d
 
a
n
d
 
t
h
e
 
l
i
s
t
 
c
a
n
n
o
t
 
a
c
c
e
p
t
 
i
t
.
 
 
T
h
e
 
w
a
y
 
t
o


a
n
s
w
e
r
 
a
 
q
u
e
s
t
i
o
n
 
a
b
o
u
t
 
a
 
t
r
a
n
s
i
t
i
o
n
 
i
s
 
t
o
 
f
i
n
d
 
a
n
 
i
m
a
g
e
 
t
h
a
t
 
h
a
s
 
b
e
e
n
 
t
h
r
o
u
g
h


i
t
,
 
a
n
d
 
a
l
l
 
t
h
r
e
e
 
i
m
a
g
e
s
 
h
e
r
e
 
h
a
v
e
 
a
 
l
i
s
t
 
w
h
o
s
e
 
w
i
n
d
o
w
 
d
o
e
s
 
n
o
t
 
s
t
a
r
t
 
a
t
 
s
l
o
t
 
0
 
—


`
x
f
s
v
4
.
i
m
g
`
'
s
 
g
r
o
u
p
s
 
1
 
a
n
d
 
3
 
s
i
t
 
a
t
 
8
5
 
a
n
d
 
2
6
 
—
 
w
h
i
c
h
 
l
o
o
k
s
 
l
i
k
e
 
a
 
l
o
n
g
 
r
e
c
o
r
d
 
o
f


b
l
o
c
k
s
 
t
h
e
 
l
i
s
t
 
h
a
s
 
h
a
n
d
e
d
 
o
u
t
.
 
 
R
u
n
n
i
n
g
 
i
t
 
s
a
y
s
 
s
o
m
e
t
h
i
n
g
 
d
i
f
f
e
r
e
n
t
:




`
`
`
t
e
x
t


i
m
a
g
e
 
 
 
 
 
 
 
 
 
 
 
 
 
g
r
o
u
p
 
 
w
i
n
d
o
w
 
 
 
 
 
 
 
 
s
l
o
t
s
 
o
u
t
s
i
d
e
 
 
 
a
 
b
-
t
r
e
e
 
n
o
d
e
 
 
 
f
r
e
e
 
s
p
a
c
e


x
f
s
v
4
.
i
m
g
 
 
 
 
 
 
 
 
 
 
1
 
 
 
 
 
(
8
5
,
 
9
0
,
 
6
)
 
 
 
 
 
 
 
 
 
8
4
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
6
2
 
 
 
 
 
 
 
 
 
 
 
 
1
4


x
f
s
v
4
.
i
m
g
 
 
 
 
 
 
 
 
 
 
3
 
 
 
 
 
(
2
6
,
 
3
3
,
 
8
)
 
 
 
 
 
 
 
 
1
2
0
 
 
 
 
 
 
 
 
 
 
 
 
 
 
1
1
9
 
 
 
 
 
 
 
 
 
 
 
 
 
0


x
f
s
_
w
r
i
t
a
b
l
e
.
i
m
g
 
 
 
a
l
l
 
 
 
(
1
,
 
4
,
 
4
)
 
 
 
 
 
 
 
 
 
 
 
0
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
-
 
 
 
 
 
 
 
 
 
 
 
 
 
-


x
f
s
_
4
k
n
.
i
m
g
 
 
 
 
 
 
 
 
a
l
l
 
 
 
(
1
,
 
4
,
 
4
)
 
 
 
 
 
 
 
 
 
 
 
0
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
-
 
 
 
 
 
 
 
 
 
 
 
 
 
-


`
`
`




T
h
e
 
t
w
o
 
i
m
a
g
e
s
 
`
m
k
f
s
.
x
f
s
`
 
p
r
o
d
u
c
e
d
 
h
a
v
e
 
*
*
n
e
v
e
r
 
c
o
n
s
u
m
e
d
 
a
 
l
i
s
t
 
e
n
t
r
y
 
a
t
 
a
l
l
*
*
,


i
n
 
a
n
y
 
o
f
 
t
h
e
i
r
 
e
i
g
h
t
 
g
r
o
u
p
s
:
 
e
v
e
r
y
 
w
i
n
d
o
w
 
i
s
 
a
t
 
s
l
o
t
 
1
 
w
i
t
h
 
f
o
u
r
 
e
n
t
r
i
e
s
 
a
n
d


n
o
t
h
i
n
g
 
o
u
t
s
i
d
e
 
i
t
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
s
t
a
t
e
 
a
 
f
r
e
s
h
l
y
 
m
a
d
e
 
f
i
l
e
 
s
y
s
t
e
m
 
i
s
 
i
n
.
 
 
S
o


t
h
e
r
e
 
i
s
 
n
o
 
e
v
i
d
e
n
c
e
 
f
r
o
m
 
t
h
e
m
 
o
f
 
w
h
a
t
 
a
 
c
o
n
s
u
m
e
d
 
e
n
t
r
y
 
b
e
c
o
m
e
s
.




T
h
e
 
o
n
l
y
 
t
r
a
c
e
 
i
s
 
i
n
 
`
x
f
s
v
4
.
i
m
g
`
,
 
w
h
i
c
h
 
i
s
 
*
*
h
a
n
d
-
b
u
i
l
t
*
*
 
b
y
 
`
s
c
r
i
p
t
s
/
m
k
i
m
g
.
s
h
`


r
a
t
h
e
r
 
t
h
a
n
 
m
a
d
e
 
b
y
 
a
 
f
i
l
e
 
s
y
s
t
e
m
.
 
 
I
t
s
 
f
o
u
r
t
e
e
n
 
f
r
e
e
-
s
p
a
c
e
 
b
l
o
c
k
s
 
a
r
e
 
a
s
 
l
i
k
e
l
y


t
o
 
b
e
 
t
h
e
 
s
c
r
i
p
t
'
s
 
d
o
i
n
g
 
a
s
 
a
 
f
i
l
e
 
s
y
s
t
e
m
'
s
,
 
a
n
d
 
t
h
i
s
 
s
u
i
t
e
 
c
a
n
n
o
t
 
s
e
t
t
l
e
 
w
h
i
c
h
.


C
a
l
l
i
n
g
 
t
h
o
s
e
 
f
o
u
r
t
e
e
n
 
o
b
s
e
r
v
a
t
i
o
n
s
 
o
f
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
w
o
u
l
d
 
b
e
 
r
e
a
d
i
n
g
 
a
 
n
u
m
b
e
r


a
s
 
a
n
 
a
n
s
w
e
r
.




T
h
a
t
 
s
e
a
r
c
h
 
c
a
m
e
 
u
p
 
e
m
p
t
y
,
 
a
n
d
 
f
o
r
 
a
 
w
h
i
l
e
 
t
h
e
 
c
o
n
c
l
u
s
i
o
n
 
w
r
i
t
t
e
n
 
h
e
r
e
 
w
a
s
 
t
h
a
t


t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
w
a
s
 
u
n
m
e
a
s
u
r
e
d
 
a
n
d
 
m
i
g
h
t
 
n
o
t
 
e
x
i
s
t
 
—
 
w
h
i
c
h
 
w
o
u
l
d
 
h
a
v
e
 
m
e
a
n
t
 
*
n
o
t
*


b
u
i
l
d
i
n
g
 
t
h
e
 
f
a
l
l
b
a
c
k
 
b
r
a
n
c
h
 
o
n
 
t
h
e
 
s
t
r
e
n
g
t
h
 
o
f
 
t
h
e
 
p
l
a
n
'
s
 
q
u
e
s
t
i
o
n
.




I
t
 
e
x
i
s
t
s
.
 
 
T
h
e
 
i
n
g
r
e
d
i
e
n
t
 
t
h
e
 
s
e
a
r
c
h
 
w
a
s
 
m
i
s
s
i
n
g
 
i
s
 
n
o
t
 
a
 
m
o
u
n
t
:
 
i
t
 
i
s
 
a
 
s
t
a
t
e


t
h
e
 
*
*
t
o
o
l
*
*
 
c
a
n
 
b
e
 
a
s
k
e
d
 
a
b
o
u
t
.
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
b
u
i
l
d
s
 
t
h
e
 
f
r
e
e
 
l
i
s
t
,
 
s
o
 
a
 
l
i
s
t


t
h
a
t
 
i
s
 
a
l
r
e
a
d
y
 
f
u
l
l
 
i
s
 
a
 
s
t
a
t
e
 
r
e
p
a
i
r
 
h
a
s
 
t
o
 
h
a
v
e
 
a
n
 
o
p
i
n
i
o
n
 
a
b
o
u
t
,
 
a
n
d
 
w
h
a
t
 
i
t


d
o
e
s
 
w
i
t
h
 
t
h
e
 
b
l
o
c
k
s
 
o
n
 
i
t
 
i
s
 
X
F
S
'
s
 
o
w
n
 
a
n
s
w
e
r
:




`
`
`
t
e
x
t


b
e
f
o
r
e
:
 
 
t
h
e
 
l
i
s
t
 
i
s
 
(
8
6
,
1
2
7
,
4
2
)
 
h
o
l
d
i
n
g
 
4
2
 
e
n
t
r
i
e
s
 
i
n
 
a
 
1
2
8
-
s
l
o
t
 
a
r
r
a
y


a
f
t
e
r
:
 
 
 
x
f
s
_
r
e
p
a
i
r
 
r
e
b
u
i
l
t
 
t
h
e
 
l
i
s
t
:
 
w
i
n
d
o
w
 
(
0
,
7
,
8
)
 
w
i
t
h
 
8
 
e
n
t
r
i
e
s


 
 
 
 
 
 
 
 
 
o
f
 
t
h
e
 
4
2
 
b
l
o
c
k
s
 
t
h
a
t
 
w
e
r
e
 
o
n
 
t
h
e
 
l
i
s
t
:


 
 
 
 
 
 
 
 
 
 
 
0
 
o
n
 
t
h
e
 
r
e
b
u
i
l
t
 
l
i
s
t
,
 
4
2
 
n
o
w
 
f
r
e
e
 
s
p
a
c
e
,
 
0
 
n
e
i
t
h
e
r


`
`
`




*
*
E
v
e
r
y
 
b
l
o
c
k
 
t
h
a
t
 
w
a
s
 
o
n
 
t
h
e
 
f
u
l
l
 
l
i
s
t
 
c
a
m
e
 
b
a
c
k
 
a
s
 
o
r
d
i
n
a
r
y
 
f
r
e
e
 
s
p
a
c
e
.
*
*
 
 
A
 
f
r
e
e


l
i
s
t
 
t
h
a
t
 
c
a
n
n
o
t
 
h
o
l
d
 
m
o
r
e
 
d
o
e
s
 
n
o
t
 
k
e
e
p
 
i
t
s
 
b
l
o
c
k
s
:
 
t
h
e
y
 
b
e
c
o
m
e
 
f
r
e
e
 
b
l
o
c
k
s
,


c
o
u
n
t
e
d
 
i
n
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
a
n
d
 
n
o
w
h
e
r
e
 
e
l
s
e
.
 
 
S
o
 
t
h
e
 
a
c
c
o
u
n
t
i
n
g
 
t
h
e
 
f
a
l
l
b
a
c
k


b
r
a
n
c
h
 
a
l
r
e
a
d
y
 
i
m
p
l
e
m
e
n
t
s
 
i
s
 
t
h
e
 
r
i
g
h
t
 
o
n
e
 
—
 
t
h
e
 
b
l
o
c
k
 
l
e
a
v
e
s
 
t
h
e
 
l
i
s
t
'
s
 
t
e
r
m
 
a
n
d


e
n
t
e
r
s
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
e
r
m
,
 
a
n
d
 
`
s
b
_
f
d
b
l
o
c
k
s
`
 
d
o
e
s
 
n
o
t
 
m
o
v
e
.




W
h
a
t
 
t
h
e
 
e
x
p
e
r
i
m
e
n
t
 
d
o
e
s
 
*
*
n
o
t
*
*
 
s
e
t
t
l
e
,
 
a
n
d
 
w
h
a
t
 
i
t
 
w
o
u
l
d
 
b
e
 
w
r
o
n
g
 
t
o
 
r
e
a
d
 
i
n
t
o


i
t
:




*
 
*
*
T
h
e
 
a
c
t
o
r
 
i
s
 
t
h
e
 
r
e
c
o
v
e
r
y
 
t
o
o
l
.
*
*
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
d
i
s
c
a
r
d
i
n
g
 
a
n
d
 
r
e
b
u
i
l
d
i
n
g
 
a


 
 
l
i
s
t
 
i
s
 
a
 
d
i
f
f
e
r
e
n
t
 
o
p
e
r
a
t
i
o
n
 
f
r
o
m
 
t
h
e
 
r
u
n
n
i
n
g
 
f
i
l
e
 
s
y
s
t
e
m
 
d
e
c
i
d
i
n
g
 
w
h
e
r
e
 
t
o
 
p
u
t


 
 
a
 
n
o
d
e
 
i
t
 
h
a
s
 
f
i
n
i
s
h
e
d
 
w
i
t
h
.
 
 
B
o
t
h
 
w
o
u
l
d
 
m
o
v
e
 
t
h
e
 
s
a
m
e
 
c
o
u
n
t
e
r
s
;
 
t
h
i
s
 
d
o
e
s
 
n
o
t


 
 
s
h
o
w
 
t
h
e
 
s
e
c
o
n
d
 
o
n
e
 
e
v
e
r
 
*
c
r
e
a
t
e
s
*
 
t
h
e
 
c
o
n
d
i
t
i
o
n
.


*
 
*
*
S
t
o
c
k
i
n
g
 
k
e
e
p
s
 
a
 
s
l
o
t
 
i
n
 
h
a
n
d
*
*
,
 
s
o
 
t
h
e
 
r
u
n
n
i
n
g
 
f
i
l
e
 
s
y
s
t
e
m
'
s
 
o
w
n
 
p
a
t
h
 
m
a
y


 
 
n
e
v
e
r
 
f
i
l
l
 
t
h
e
 
l
i
s
t
 
a
t
 
a
l
l
.
 
 
`
a
p
p
e
n
d
_
t
o
_
t
h
e
_
f
r
e
e
_
l
i
s
t
`
 
s
t
o
p
s
 
o
n
e
 
s
l
o
t
 
s
h
o
r
t
 
o
f


 
 
t
h
e
 
e
n
d
 
o
n
 
p
u
r
p
o
s
e
,
 
w
h
i
c
h
 
i
s
 
w
h
y
 
f
i
l
l
i
n
g
 
t
h
i
s
 
o
n
e
 
t
o
o
k
 
a
 
d
e
l
i
b
e
r
a
t
e


 
 
t
a
k
e
-
a
n
d
-
g
i
v
e
-
b
a
c
k
 
r
a
t
h
e
r
 
t
h
a
n
 
4
3
 
r
o
u
n
d
s
 
o
f
 
f
r
e
e
i
n
g
.




T
h
e
 
l
e
s
s
o
n
 
w
o
r
t
h
 
k
e
e
p
i
n
g
 
i
s
 
a
b
o
u
t
 
m
e
t
h
o
d
 
r
a
t
h
e
r
 
t
h
a
n
 
a
b
o
u
t
 
t
h
e
 
f
r
e
e
 
l
i
s
t
:
 
a


q
u
e
s
t
i
o
n
 
a
b
o
u
t
 
a
 
t
r
a
n
s
i
t
i
o
n
 
l
o
o
k
e
d
 
f
o
r
 
e
v
i
d
e
n
c
e
 
o
f
 
t
h
e
 
t
r
a
n
s
i
t
i
o
n
 
*
h
a
p
p
e
n
i
n
g


n
a
t
u
r
a
l
l
y
*
,
 
a
n
d
 
f
o
u
n
d
 
n
o
n
e
,
 
a
n
d
 
n
e
a
r
l
y
 
r
e
c
o
r
d
e
d
 
"
i
t
 
d
o
e
s
 
n
o
t
 
e
x
i
s
t
"
.
 
 
T
h
e


q
u
e
s
t
i
o
n
 
w
a
s
 
a
n
s
w
e
r
a
b
l
e
 
b
y
 
p
u
t
t
i
n
g
 
a
 
f
i
l
e
 
s
y
s
t
e
m
 
i
n
 
t
h
a
t
 
s
t
a
t
e
 
a
n
d
 
a
s
k
i
n
g
.




#
#
#
 
O
n
e
 
d
i
f
f
e
r
e
n
c
e
 
f
r
o
m
 
X
F
S
,
 
r
e
c
o
r
d
e
d
 
b
e
c
a
u
s
e
 
i
t
 
w
a
s
 
m
e
a
s
u
r
e
d




*
*
T
h
e
 
s
l
o
t
s
 
a
 
l
i
s
t
 
h
a
s
 
p
a
s
s
e
d
 
o
v
e
r
 
s
t
i
l
l
 
h
o
l
d
 
t
h
e
i
r
 
b
l
o
c
k
 
n
u
m
b
e
r
s
.
*
*
 
 
I
n
 
g
r
o
u
p
 
1


o
f
 
`
x
f
s
v
4
.
i
m
g
`
,
 
8
4
 
o
f
 
t
h
e
 
1
2
8
 
s
l
o
t
s
 
a
r
e
 
n
o
n
-
n
u
l
l
 
o
u
t
s
i
d
e
 
a
 
s
i
x
-
e
n
t
r
y
 
w
i
n
d
o
w
,
 
a
n
d


6
2
 
o
f
 
t
h
e
m
 
n
a
m
e
 
b
l
o
c
k
s
 
t
h
a
t
 
a
r
e
 
b
-
t
r
e
e
 
n
o
d
e
s
 
a
t
 
t
h
i
s
 
m
o
m
e
n
t
.
 
 
`
A
g
f
l
:
:
t
a
k
e
_
f
r
o
n
t
`


n
u
l
l
s
 
t
h
e
 
s
l
o
t
 
i
t
 
t
a
k
e
s
;
 
X
F
S
 
e
v
i
d
e
n
t
l
y
 
d
o
e
s
 
n
o
t
.




B
o
t
h
 
a
r
e
 
s
a
f
e
,
 
a
n
d
 
f
o
r
 
t
h
e
 
s
a
m
e
 
r
e
a
s
o
n
:
 
t
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r
 
s
a
y
s
 
w
h
i
c
h
 
s
l
o
t
s
 
a
r
e


l
i
v
e
,
 
s
o
 
a
 
v
a
l
u
e
 
o
u
t
s
i
d
e
 
t
h
e
 
w
i
n
d
o
w
 
i
s
 
a
 
l
e
f
t
o
v
e
r
 
a
n
d
 
n
o
t
 
a
n
 
o
f
f
e
r
.
 
 
T
h
a
t
 
i
s
 
w
h
a
t


`
A
g
f
l
:
:
w
i
n
d
o
w
_
h
o
l
d
s
`
 
a
s
s
u
m
e
s
 
w
h
e
n
 
i
t
 
r
e
f
u
s
e
s
 
a
 
d
u
p
l
i
c
a
t
e
,
 
a
n
d
 
w
h
y
 
n
o
t
h
i
n
g
 
i
n
 
t
h
i
s


c
o
d
e
 
s
c
a
n
s
 
t
h
e
 
a
r
r
a
y
.
 
 
N
u
l
l
i
n
g
 
i
s
 
k
e
p
t
,
 
b
e
c
a
u
s
e
 
i
t
 
m
a
k
e
s
 
a
 
b
l
a
n
k
 
s
l
o
t


d
i
s
t
i
n
g
u
i
s
h
a
b
l
e
 
f
r
o
m
 
a
 
s
t
a
l
e
 
o
n
e
 
f
o
r
 
a
n
y
t
h
i
n
g
 
t
h
a
t
 
e
v
e
r
 
r
e
a
d
s
 
t
h
e
 
a
r
r
a
y
 
w
i
t
h
o
u
t


t
h
e
 
h
e
a
d
e
r
 
—
 
w
h
i
c
h
 
i
s
 
t
h
e
 
m
i
s
t
a
k
e
 
t
h
a
t
 
t
h
i
s
 
c
o
d
e
b
a
s
e
 
h
a
s
 
a
l
r
e
a
d
y
 
m
a
d
e
 
o
n
c
e
.




#
#
 
P
h
a
s
e
 
h
i
s
t
o
r
y




W
h
a
t
 
e
a
c
h
 
p
h
a
s
e
 
w
a
s
,
 
a
n
d
 
w
h
a
t
 
i
t
 
f
o
u
n
d
.
 
 
T
h
e
 
m
e
a
s
u
r
e
m
e
n
t
s
 
a
r
e
 
k
e
p
t
 
b
e
c
a
u
s
e
 
t
h
e
y


a
r
e
 
e
x
p
e
n
s
i
v
e
 
t
o
 
m
a
k
e
 
a
g
a
i
n
;
 
t
h
e
 
n
a
r
r
a
t
i
o
n
 
i
s
 
n
o
t
 
r
e
p
e
a
t
e
d
.




#
#
#
 
P
h
a
s
e
s
 
1
–
3
 
—
 
d
e
v
i
c
e
,
 
c
a
c
h
e
,
 
t
r
a
n
s
a
c
t
i
o
n
s




`
B
l
o
c
k
D
e
v
i
c
e
`
 
i
s
 
a
n
 
o
w
n
e
d
 
h
a
n
d
l
e
 
w
i
t
h
 
p
o
s
i
t
i
o
n
a
l
 
I
/
O
 
a
n
d
 
n
o
 
b
u
f
f
e
r
i
n
g
,
 
a
n
d


`
B
l
o
c
k
R
e
a
d
e
r
`
 
w
a
s
 
r
e
f
a
c
t
o
r
e
d
 
o
n
t
o
 
i
t
 
s
o
 
t
h
e
 
r
e
a
d
 
p
a
t
h
 
i
s
 
u
n
c
h
a
n
g
e
d
.
 
 
O
n
e
 
h
a
n
d
l
e


a
n
d
 
o
n
e
 
`
p
r
e
a
d
`
/
`
p
w
r
i
t
e
`
 
p
a
t
h
 
m
e
a
n
s
 
t
h
e
 
f
i
l
e
 
s
y
s
t
e
m
 
c
a
n
 
n
e
v
e
r
 
h
o
l
d
 
t
w
o


w
r
i
t
a
b
l
e
 
c
o
p
i
e
s
 
o
f
 
t
h
e
 
s
a
m
e
 
p
h
y
s
i
c
a
l
 
b
l
o
c
k
.
 
 
`
B
l
o
c
k
C
a
c
h
e
`
 
h
o
l
d
s
 
b
l
o
c
k
s
 
w
i
t
h
 
a
n


e
x
p
l
i
c
i
t
 
s
t
a
t
e
 
(
`
C
l
e
a
n
`
,
 
`
D
i
r
t
y
`
,
 
`
L
o
g
g
e
d
`
,
 
`
C
o
m
m
i
t
t
e
d
`
)
;
 
t
h
e
 
l
a
s
t
 
t
w
o
 
a
r
e
 
n
o
t


y
e
t
 
p
r
o
d
u
c
e
d
 
b
y
 
a
n
y
t
h
i
n
g
 
a
n
d
 
e
x
i
s
t
 
s
o
 
t
h
e
 
j
o
u
r
n
a
l
 
n
e
e
d
 
n
o
t
 
c
h
a
n
g
e
 
t
h
e
 
i
n
t
e
r
f
a
c
e
.


`
T
r
a
n
s
a
c
t
i
o
n
`
 
i
s
 
t
h
e
 
o
n
l
y
 
w
a
y
 
t
h
e
 
i
m
a
g
e
 
c
h
a
n
g
e
s
,
 
w
i
t
h
 
`
R
e
a
d
O
n
l
y
`
 
a
n
d
 
`
D
i
r
e
c
t
`


c
o
m
m
i
t
 
m
o
d
e
s
 
b
e
h
i
n
d
 
o
n
e
 
`
c
o
m
m
i
t
`
,
 
a
n
d
 
a
n
 
u
n
c
o
m
m
i
t
t
e
d
 
t
r
a
n
s
a
c
t
i
o
n
 
l
e
a
v
i
n
g
 
t
h
e


i
m
a
g
e
 
e
x
a
c
t
l
y
 
a
s
 
i
t
 
w
a
s
.




#
#
#
 
P
h
a
s
e
 
4
–
6
 
—
 
i
n
o
d
e
s
,
 
e
x
t
e
n
t
s
,
 
t
h
e
 
w
r
i
t
e
 
p
a
t
h




`
R
a
w
D
i
n
o
d
e
`
 
o
w
n
s
 
a
n
 
i
n
o
d
e
'
s
 
e
x
a
c
t
 
b
y
t
e
s
 
w
i
t
h
 
t
y
p
e
d
 
a
c
c
e
s
s
o
r
s
,
 
s
o
 
t
h
e
 
p
a
r
s
e
r
 
a
n
d


t
h
e
 
s
e
r
i
a
l
i
z
e
r
 
c
a
n
n
o
t
 
d
r
i
f
t
 
a
p
a
r
t
.
 
 
`
E
x
t
e
n
t
M
a
p
`
 
i
s
 
t
h
e
 
o
n
e
 
p
l
a
c
e
 
t
h
a
t
 
a
n
s
w
e
r
s


w
h
e
r
e
 
a
 
f
i
l
e
'
s
 
l
o
g
i
c
a
l
 
b
l
o
c
k
 
l
i
v
e
s
,
 
a
n
d
 
b
o
t
h
 
t
h
e
 
r
e
a
d
 
a
n
d
 
t
h
e
 
w
r
i
t
e
 
p
a
t
h
 
u
s
e


i
t
.
 
 
`
F
s
C
a
p
a
b
i
l
i
t
i
e
s
`
 
s
e
p
a
r
a
t
e
s
 
w
h
a
t
 
i
s
 
s
u
p
p
o
r
t
e
d
 
f
o
r
 
r
e
a
d
i
n
g
 
f
r
o
m
 
w
h
a
t
 
i
s


s
u
p
p
o
r
t
e
d
 
f
o
r
 
w
r
i
t
i
n
g
,
 
a
n
d
 
a
 
r
e
a
d
-
w
r
i
t
e
 
m
o
u
n
t
 
o
f
 
a
n
 
i
m
a
g
e
 
w
i
t
h
 
a
 
f
e
a
t
u
r
e
 
i
t


c
a
n
n
o
t
 
m
a
i
n
t
a
i
n
 
i
s
 
r
e
f
u
s
e
d
 
b
y
 
n
a
m
e
.




#
#
#
 
P
h
a
s
e
 
7
 
—
 
a
l
l
o
c
a
t
i
o
n
 
g
r
o
u
p
s




T
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r
,
 
t
h
e
 
g
r
o
u
p
 
i
n
o
d
e
 
h
e
a
d
e
r
,
 
t
h
e
 
f
r
e
e
 
l
i
s
t
,
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s


a
n
d
 
t
h
e
 
t
r
e
e
 
o
f
 
u
s
e
d
 
i
n
o
d
e
 
n
u
m
b
e
r
s
,
 
a
l
l
 
k
e
p
t
 
a
s
 
t
h
e
i
r
 
o
w
n
 
b
y
t
e
s
 
a
n
d
 
c
h
a
n
g
e
d
 
i
n


p
l
a
c
e
 
s
o
 
t
h
a
t
 
f
i
e
l
d
s
 
t
h
i
s
 
c
o
d
e
 
h
a
s
 
n
o
 
o
p
i
n
i
o
n
 
a
b
o
u
t
 
s
u
r
v
i
v
e
 
a
 
w
r
i
t
e
 
b
y


c
o
n
s
t
r
u
c
t
i
o
n
.




F
o
u
r
 
t
h
i
n
g
s
 
a
b
o
u
t
 
t
h
e
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s
 
c
a
m
e
 
o
u
t
 
o
f
 
e
x
p
e
r
i
m
e
n
t
s
 
a
n
d
 
a
r
e
 
w
r
i
t
t
e
n


i
n
t
o
 
t
h
e
 
c
o
d
e
 
r
a
t
h
e
r
 
t
h
a
n
 
i
n
t
o
 
s
o
m
e
o
n
e
'
s
 
h
e
a
d
:




*
 
*
*
A
 
l
e
a
f
 
h
o
l
d
s
 
r
e
c
o
r
d
s
 
a
n
d
 
n
o
t
h
i
n
g
 
e
l
s
e
.
*
*
 
 
6
2
 
o
f
 
t
h
e
m
 
f
i
l
l
 
t
h
e
 
r
e
g
i
o
n
 
o
f
 
a


 
 
5
1
2
-
b
y
t
e
 
b
l
o
c
k
 
e
x
a
c
t
l
y
,
 
s
o
 
a
 
t
r
e
e
'
s
 
k
e
y
s
 
l
i
v
e
 
o
n
l
y
 
i
n
 
i
t
s
 
i
n
t
e
r
i
o
r
 
n
o
d
e
s
.
 
 
T
h
e


 
 
f
i
r
s
t
 
v
e
r
s
i
o
n
 
o
f
 
t
h
e
 
l
e
a
f
 
w
r
i
t
e
r
 
k
e
p
t
 
a
 
s
e
c
o
n
d
 
a
r
r
a
y
 
o
f
 
k
e
y
s
 
t
h
e
r
e
 
a
n
d
 
w
r
o
t
e


 
 
p
a
s
t
 
t
h
e
 
e
n
d
 
o
f
 
t
h
e
 
b
l
o
c
k
.


*
 
*
*
A
 
l
e
a
f
 
b
e
l
o
w
 
h
a
l
f
 
i
s
 
a
 
f
a
u
l
t
,
 
n
o
t
 
a
 
s
h
a
p
e
.
*
*
 
 
`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
f
u
s
e
s
 
o
n
e


 
 
h
o
l
d
i
n
g
 
f
e
w
e
r
 
t
h
a
n
 
3
1
 
r
e
c
o
r
d
s
.
 
 
A
n
 
e
a
r
l
i
e
r
 
v
e
r
s
i
o
n
 
o
f
 
t
h
i
s
 
w
o
r
k
 
r
e
a
d
 
t
h
e
 
B
+
t
r
e
e


 
 
d
o
c
u
m
e
n
t
a
t
i
o
n
'
s
 
"
s
h
o
u
l
d
 
r
e
b
a
l
a
n
c
e
"
 
a
s
 
a
 
p
o
l
i
c
y
 
a
n
d
 
d
r
o
p
p
e
d
 
m
e
r
g
i
n
g
 
f
r
o
m
 
t
h
e


 
 
t
a
k
e
 
p
a
t
h
;
 
`
x
f
s
_
r
e
p
a
i
r
`
 
i
s
 
a
b
o
u
t
 
w
h
a
t
 
X
F
S
 
w
i
l
l
 
*
a
c
c
e
p
t
*
,
 
a
n
d
 
t
h
e
 
t
w
o
 
a
r
e
 
n
o
t


 
 
t
h
e
 
s
a
m
e
 
t
h
i
n
g
.


*
 
*
*
A
 
n
o
d
e
'
s
 
r
e
c
o
r
d
 
c
o
u
n
t
 
l
i
v
e
s
 
i
n
 
t
w
o
 
p
l
a
c
e
s
*
*
 
—
 
t
h
e
 
b
y
t
e
s
 
a
n
d
 
t
h
e
 
f
i
e
l
d
 
t
h
e


 
 
s
t
r
u
c
t
 
w
a
s
 
b
u
i
l
t
 
w
i
t
h
 
—
 
a
n
d
 
t
h
e
y
 
h
a
v
e
 
t
o
 
m
o
v
e
 
t
o
g
e
t
h
e
r
.
 
 
T
h
i
s
 
i
s
 
i
n
v
i
s
i
b
l
e
 
i
n


 
 
a
 
t
e
s
t
 
t
h
a
t
 
r
e
a
d
s
 
a
 
n
o
d
e
 
b
a
c
k
 
t
h
r
o
u
g
h
 
t
h
e
 
s
t
r
u
c
t
 
t
h
a
t
 
w
r
o
t
e
 
i
t
.


*
 
*
*
T
h
e
 
t
w
o
 
t
r
e
e
s
 
r
e
c
o
r
d
 
t
h
e
 
s
a
m
e
 
r
u
n
s
,
 
n
o
t
 
m
e
r
e
l
y
 
t
h
e
 
s
a
m
e
 
b
l
o
c
k
s
.
*
*
 
 
I
n
 
e
v
e
r
y


 
 
g
r
o
u
p
 
o
f
 
`
x
f
s
v
4
.
i
m
g
`
,
 
i
n
c
l
u
d
i
n
g
 
o
n
e
 
o
f
 
1
7
1
3
 
r
e
c
o
r
d
s
,
 
t
h
e
 
b
n
o
 
a
n
d
 
c
n
t
 
t
r
e
e
s


 
 
h
o
l
d
 
i
d
e
n
t
i
c
a
l
 
r
e
c
o
r
d
 
s
e
t
s
.
 
 
S
o
 
t
h
e
 
t
r
e
e
 
k
e
y
e
d
 
b
y
 
s
t
a
r
t
 
b
l
o
c
k
 
*
d
e
c
i
d
e
s
*
 
w
h
a
t
 
a


 
 
f
r
e
e
 
m
e
a
n
s
 
—
 
t
h
e
r
e
 
a
 
r
u
n
'
s
 
n
e
i
g
h
b
o
u
r
s
 
a
l
o
n
g
 
t
h
e
 
g
r
o
u
p
'
s
 
b
l
o
c
k
s
 
a
r
e
 
i
t
s


 
 
n
e
i
g
h
b
o
u
r
s
 
i
n
 
t
h
e
 
t
r
e
e
'
s
 
o
r
d
e
r
 
—
 
a
n
d
 
t
h
e
 
t
r
e
e
 
k
e
y
e
d
 
b
y
 
l
e
n
g
t
h
 
i
s
 
b
r
o
u
g
h
t
 
t
o


 
 
t
h
a
t
 
a
n
s
w
e
r
.
 
 
L
e
t
t
i
n
g
 
e
a
c
h
 
d
e
c
i
d
e
 
f
o
r
 
i
t
s
e
l
f
 
i
s
 
w
h
a
t
 
p
r
o
d
u
c
e
d
 
a
 
d
i
v
e
r
g
e
n
c
e


 
 
t
h
a
t
 
o
n
l
y
 
s
h
o
w
e
d
 
u
p
 
a
s
 
a
 
t
a
k
e
 
f
i
n
d
i
n
g
 
a
 
r
e
c
o
r
d
 
i
n
 
o
n
e
 
t
r
e
e
 
a
n
d
 
n
o
t
 
i
n
 
t
h
e


 
 
o
t
h
e
r
.




#
#
#
 
S
i
b
l
i
n
g
 
l
i
n
k
s
 
a
r
e
 
s
t
r
u
c
t
u
r
e
,
 
n
o
t
 
n
a
v
i
g
a
t
i
o
n




S
i
b
l
i
n
g
 
p
o
i
n
t
e
r
s
 
a
r
e
 
l
i
v
e
 
s
t
r
u
c
t
u
r
a
l
 
m
e
t
a
d
a
t
a
:
 
X
F
S
 
w
a
n
t
s
 
t
h
e
m
 
t
o
 
n
a
m
e
 
v
a
l
i
d


b
l
o
c
k
s
 
a
t
 
t
h
e
 
s
a
m
e
 
l
e
v
e
l
,
 
t
h
e
 
c
h
a
i
n
 
t
o
 
b
e
 
b
i
d
i
r
e
c
t
i
o
n
a
l
,
 
a
n
d
 
t
h
e
 
r
o
o
t
 
t
o
 
h
a
v
e


n
o
n
e
.
 
 
D
r
o
p
p
i
n
g
 
a
 
b
l
o
c
k
 
i
n
 
t
h
e
 
m
i
d
d
l
e
 
o
f
 
a
 
l
e
v
e
l
 
t
h
e
r
e
f
o
r
e
 
h
a
s
 
t
o
 
r
e
l
i
n
k
 
*
b
o
t
h
*


s
i
d
e
s
,
 
a
n
d
 
o
n
l
y
 
t
h
e
 
b
l
o
c
k
s
 
s
t
i
l
l
 
i
n
 
t
h
e
 
t
r
e
e
 
—
 
t
h
e
 
b
l
o
c
k
 
b
e
i
n
g
 
d
r
o
p
p
e
d
 
i
s
 
l
e
f
t


a
s
 
i
t
 
i
s
,
 
b
e
c
a
u
s
e
 
i
t
s
 
c
o
n
t
e
n
t
s
 
a
r
e
 
n
o
 
l
o
n
g
e
r
 
p
a
r
t
 
o
f
 
t
h
e
 
t
r
e
e
'
s
 
g
r
a
p
h
.


`
c
h
e
c
k
_
s
i
b
l
i
n
g
_
c
h
a
i
n
s
`
 
r
u
n
s
 
a
f
t
e
r
 
e
v
e
r
y
 
m
u
t
a
t
i
o
n
 
r
a
t
h
e
r
 
t
h
a
n
 
o
n
c
e
 
a
t
 
t
h
e
 
e
n
d
,


a
n
d
 
w
a
s
 
c
o
n
f
i
r
m
e
d
 
t
o
 
f
a
i
l
 
w
h
e
n
 
e
a
c
h
 
o
f
 
t
h
e
 
t
h
r
e
e
 
f
a
i
l
u
r
e
 
m
o
d
e
s
 
i
s
 
i
n
t
r
o
d
u
c
e
d
 
o
n


p
u
r
p
o
s
e
,
 
w
h
i
c
h
 
i
s
 
t
h
e
 
o
n
l
y
 
w
a
y
 
t
o
 
k
n
o
w
 
a
 
c
h
e
c
k
 
o
f
 
t
h
a
t
 
k
i
n
d
 
i
s
 
d
o
i
n
g
 
a
n
y
t
h
i
n
g
.




#
#
#
 
F
i
e
l
d
s
 
d
e
r
i
v
e
d
 
f
r
o
m
 
a
 
s
u
m
,
 
g
u
a
r
d
e
d
 
b
y
 
a
 
t
e
s
t
 
o
n
 
a
 
c
o
u
n
t




G
r
o
w
i
n
g
 
a
 
f
i
l
e
 
a
t
 
i
t
s
 
e
n
d
 
l
e
f
t
 
t
h
e
 
i
n
o
d
e
'
s
 
b
l
o
c
k
 
c
o
u
n
t
 
s
t
a
l
e
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
`


s
a
i
d
 
`
b
a
d
 
n
b
l
o
c
k
s
 
1
2
8
 
f
o
r
 
i
n
o
d
e
 
3
7
,
 
w
o
u
l
d
 
r
e
s
e
t
 
t
o
 
1
5
2
`
.
 
 
T
h
e
 
g
u
a
r
d
 
c
h
e
c
k
e
d
 
t
h
e


*
n
u
m
b
e
r
 
o
f
 
e
x
t
e
n
t
s
*
 
r
a
t
h
e
r
 
t
h
a
n
 
t
h
e
 
s
u
m
 
o
f
 
t
h
e
i
r
 
l
e
n
g
t
h
s
,
 
w
h
i
c
h
 
i
s
 
w
h
a
t
 
t
h
e


f
i
e
l
d
 
m
e
a
n
s
 
—
 
a
n
d
 
b
e
c
a
u
s
e
 
a
 
f
i
l
e
 
g
r
o
w
n
 
a
t
 
i
t
s
 
e
n
d
 
l
a
n
d
s
 
b
e
s
i
d
e
 
i
t
s
 
o
w
n
 
l
a
s
t


b
l
o
c
k
,
 
t
h
e
 
n
e
w
 
b
l
o
c
k
s
 
a
r
e
 
*
j
o
i
n
e
d
*
 
t
o
 
a
n
 
e
x
t
e
n
t
 
a
l
r
e
a
d
y
 
t
h
e
r
e
,
 
s
o
 
t
h
e
 
e
x
t
e
n
t


c
o
u
n
t
 
d
o
e
s
 
n
o
t
 
m
o
v
e
 
w
h
i
l
e
 
t
w
e
n
t
y
-
f
o
u
r
 
m
o
r
e
 
b
l
o
c
k
s
 
a
r
e
 
c
o
v
e
r
e
d
.
 
 
T
h
e
 
g
u
a
r
d
 
w
a
s


i
n
v
e
r
t
e
d
 
f
o
r
 
t
h
e
 
m
o
s
t
 
o
r
d
i
n
a
r
y
 
c
a
s
e
 
t
h
e
r
e
 
i
s
.




#
#
#
 
T
h
e
 
s
u
p
e
r
b
l
o
c
k
 
k
e
e
p
s
 
i
t
s
 
o
w
n
 
c
o
u
n
t




`
s
b
_
f
d
b
l
o
c
k
s
 
9
0
6
2
4
,
 
c
o
u
n
t
e
d
 
9
0
6
0
0
`
 
w
a
s
 
t
h
e
 
l
a
s
t
 
t
h
i
n
g
 
b
e
t
w
e
e
n
 
a
 
g
r
o
w
n
 
f
i
l
e
 
a
n
d


a
n
 
i
m
a
g
e
 
r
e
p
a
i
r
 
w
o
u
l
d
 
a
c
c
e
p
t
.
 
 
T
h
e
 
g
r
o
u
p
'
s
 
h
e
a
d
e
r
 
h
a
d
 
b
e
e
n
 
u
p
d
a
t
e
d
 
c
o
r
r
e
c
t
l
y
 
a
n
d


`
x
f
s
_
d
b
`
 
r
e
a
d
 
t
h
e
 
t
r
e
e
s
 
b
a
c
k
 
w
i
t
h
 
t
h
e
 
s
a
m
e
 
t
o
t
a
l
,
 
s
o
 
t
h
e
 
d
i
s
a
g
r
e
e
m
e
n
t
 
w
a
s
 
t
h
e


s
u
p
e
r
b
l
o
c
k
'
s
 
o
w
n
 
c
o
u
n
t
,
 
w
h
i
c
h
 
n
o
t
h
i
n
g
 
w
a
s
 
u
p
d
a
t
i
n
g
.
 
 
T
h
e
 
f
i
e
l
d
 
i
s
 
*
p
a
t
c
h
e
d
*
 
i
n
t
o


t
h
e
 
f
i
r
s
t
 
s
e
c
t
o
r
'
s
 
b
y
t
e
s
 
r
a
t
h
e
r
 
t
h
a
n
 
r
e
b
u
i
l
t
 
f
r
o
m
 
t
h
e
 
p
a
r
s
e
d
 
s
t
r
u
c
t
,
 
s
o
 
a
 
w
r
i
t
e


i
n
t
o
 
a
 
b
l
o
c
k
 
t
h
e
 
f
i
r
s
t
 
g
r
o
u
p
'
s
 
h
e
a
d
e
r
s
 
s
h
a
r
e
 
c
a
n
n
o
t
 
t
a
k
e
 
t
h
e
m
 
w
i
t
h
 
i
t
.
 
 
W
h
a
t
 
i
t


i
s
 
a
 
c
o
u
n
t
 
*
o
f
*
 
i
s
 
n
o
w
 
m
e
a
s
u
r
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
a
s
s
u
m
e
d
;
 
s
e
e


[
t
h
e
 
t
h
r
e
e
 
t
e
r
m
s
]
(
#
t
h
e
-
s
u
p
e
r
b
l
o
c
k
s
-
f
r
e
e
-
c
o
u
n
t
-
h
a
s
-
t
h
r
e
e
-
t
e
r
m
s
-
a
n
d
-
a
l
l
-
t
h
r
e
e
-
a
r
e
-
m
e
a
s
u
r
e
d
)
.




#
#
#
 
F
r
e
e
i
n
g
 
b
l
o
c
k
s
,
 
a
n
d
 
w
h
a
t
 
i
t
 
c
o
s
t
 
t
o
 
g
e
t
 
r
i
g
h
t




F
r
e
e
i
n
g
 
i
s
 
a
l
l
o
c
a
t
i
n
g
 
b
a
c
k
w
a
r
d
s
,
 
w
i
t
h
 
t
h
e
 
s
a
m
e
 
o
b
l
i
g
a
t
i
o
n
s
:
 
b
o
t
h
 
t
r
e
e
s
 
r
e
c
o
r
d


t
h
e
 
r
u
n
,
 
t
h
e
 
g
r
o
u
p
'
s
 
t
w
o
 
s
u
m
m
a
r
i
e
s
 
f
o
l
l
o
w
,
 
a
n
d
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
'
s
 
t
o
t
a
l
 
f
o
l
l
o
w
s
.


T
h
r
e
e
 
d
e
f
e
c
t
s
 
w
e
r
e
 
e
a
c
h
 
f
o
u
n
d
 
o
n
l
y
 
b
y
 
t
h
e
 
t
o
o
l
:




*
 
f
r
e
e
i
n
g
 
b
l
o
c
k
s
 
t
h
a
t
 
w
e
r
e
 
*
*
a
l
r
e
a
d
y
 
f
r
e
e
*
*
 
a
d
d
e
d
 
a
 
s
e
c
o
n
d
 
c
o
p
y
 
o
f
 
t
h
e
m
,
 
a
n
d


 
 
r
e
p
a
i
r
 
s
a
i
d
 
`
o
u
t
-
o
f
-
o
r
d
e
r
 
b
n
o
 
b
t
r
e
e
 
r
e
c
o
r
d
 
2
 
(
5
7
1
 
2
)
`
 
a
n
d


 
 
`
b
l
o
c
k
 
(
0
,
5
7
1
-
5
7
2
)
 
m
u
l
t
i
p
l
y
 
c
l
a
i
m
e
d
 
b
y
 
b
n
o
 
s
p
a
c
e
 
t
r
e
e
`
;


*
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
 
w
a
s
 
m
o
v
e
d
 
b
y
 
h
o
w
e
v
e
r
 
m
a
n
y
 
b
l
o
c
k
s
 
w
e
r
e
 
*
a
s
k
e
d
*
 
t
o
 
b
e
 
f
r
e
e
d


 
 
r
a
t
h
e
r
 
t
h
a
n
 
b
y
 
h
o
w
e
v
e
r
 
m
a
n
y
 
t
h
e
 
g
r
o
u
p
'
s
 
f
r
e
e
 
s
p
a
c
e
 
a
c
t
u
a
l
l
y
 
g
r
e
w
 
b
y
,
 
w
h
i
c
h


 
 
s
h
o
w
e
d
 
u
p
 
a
s
 
`
s
b
_
f
d
b
l
o
c
k
s
 
9
0
6
2
6
,
 
c
o
u
n
t
e
d
 
9
0
6
2
4
`
;


*
 
t
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r
 
w
a
s
 
w
r
i
t
t
e
n
 
t
w
i
c
e
 
i
n
 
o
n
e
 
f
r
e
e
,
 
s
o
 
a
 
s
p
l
i
t
 
i
n
 
b
e
t
w
e
e
n
 
t
h
a
t


 
 
m
o
v
e
d
 
t
h
e
 
f
r
e
e
 
l
i
s
t
 
w
i
n
d
o
w
 
h
a
d
 
i
t
s
 
w
o
r
k
 
u
n
d
o
n
e
 
a
n
d
 
t
h
e
 
w
i
n
d
o
w
 
w
a
s
 
l
e
f
t
 
n
a
m
i
n
g


 
 
a
 
s
l
o
t
 
t
h
a
t
 
h
a
d
 
b
e
e
n
 
e
m
p
t
i
e
d
.




#
#
#
 
T
h
e
 
i
n
o
d
e
 
a
l
l
o
c
a
t
i
o
n
 
r
e
c
o
r
d
 
d
e
c
i
d
e
s
,
 
n
o
t
 
t
h
e
 
s
l
o
t
s




A
 
s
l
o
t
 
b
e
i
n
g
 
p
h
y
s
i
c
a
l
l
y
 
u
n
u
s
e
d
 
d
o
e
s
 
n
o
t
 
m
a
k
e
 
i
t
 
a
l
l
o
c
a
t
a
b
l
e
.
 
 
T
h
e
 
g
r
o
u
p
'
s
 
t
r
e
e


o
f
 
u
s
e
d
 
i
n
o
d
e
 
n
u
m
b
e
r
s
 
i
s
 
t
h
e
 
o
n
l
y
 
t
h
i
n
g
 
t
h
a
t
 
s
a
y
s
 
w
h
i
c
h
 
c
h
u
n
k
s
 
e
x
i
s
t
 
—
 
t
h
e
s
e


i
m
a
g
e
s
 
s
p
a
c
e
 
t
h
e
i
r
 
r
e
c
o
r
d
s
 
1
6
0
 
i
n
o
d
e
s
 
a
p
a
r
t
 
r
a
t
h
e
r
 
t
h
a
n
 
6
4
,
 
s
o
 
a
 
c
h
u
n
k
 
n
u
m
b
e
r


w
o
r
k
e
d
 
o
u
t
 
f
r
o
m
 
t
h
e
 
i
n
o
d
e
 
n
u
m
b
e
r
 
a
l
o
n
e
 
n
a
m
e
s
 
a
 
h
o
l
e
 
—
 
a
n
d
 
a
 
c
h
u
n
k
'
s
 
6
4
-
b
i
t
 
m
a
s
k


i
s
 
t
h
e
 
o
n
l
y
 
t
h
i
n
g
 
t
h
a
t
 
s
a
y
s
 
w
h
i
c
h
 
o
f
 
i
t
s
 
i
n
o
d
e
s
 
a
r
e
 
f
r
e
e
.
 
 
T
h
e
 
t
e
s
t
 
b
l
a
n
k
s


e
v
e
r
y
 
c
h
u
n
k
 
m
a
s
k
 
i
n
 
a
 
r
e
a
l
 
g
r
o
u
p
 
a
n
d
 
r
e
q
u
i
r
e
s
 
a
l
l
o
c
a
t
i
o
n
 
t
o
 
d
e
c
l
i
n
e
 
w
h
i
l
e
 
m
o
s
t


o
f
 
i
t
s
 
s
l
o
t
s
 
a
r
e
 
d
e
m
o
n
s
t
r
a
b
l
y
 
u
n
t
o
u
c
h
e
d
.




A
l
l
o
c
a
t
i
n
g
 
f
r
o
m
 
a
n
 
e
x
i
s
t
i
n
g
 
c
h
u
n
k
 
i
s
 
f
o
u
r
 
m
o
v
e
s
 
i
n
 
o
n
e
 
t
r
a
n
s
a
c
t
i
o
n
:
 
c
l
e
a
r
 
t
h
e


l
o
w
e
s
t
 
s
e
t
 
b
i
t
,
 
t
a
k
e
 
t
h
e
 
c
o
u
n
t
 
*
b
e
s
i
d
e
 
i
t
*
 
d
o
w
n
 
b
y
 
o
n
e
,
 
t
a
k
e
 
t
h
e
 
g
r
o
u
p
'
s
 
f
r
e
e


i
n
o
d
e
 
c
o
u
n
t
 
d
o
w
n
 
b
y
 
o
n
e
,
 
a
n
d
 
t
a
k
e
 
`
s
b
_
i
f
r
e
e
`
 
d
o
w
n
 
b
y
 
o
n
e
.
 
 
`
s
b
_
i
f
r
e
e
`
 
w
a
s


m
e
a
s
u
r
e
d
 
t
o
 
b
e
 
t
h
e
 
e
x
a
c
t
 
s
u
m
 
o
f
 
t
h
e
 
g
r
o
u
p
s
'
 
f
r
e
e
 
i
n
o
d
e
 
c
o
u
n
t
s
 
o
n
 
b
o
t
h
 
r
e
f
e
r
e
n
c
e


i
m
a
g
e
s
,
 
s
o
 
t
h
e
r
e
 
i
s
 
n
o
 
s
e
p
a
r
a
t
e
 
t
e
r
m
 
t
o
 
d
e
c
i
d
e
 
o
n
.
 
 
`
a
g
i
_
n
e
w
i
n
o
`
 
d
o
e
s
 
n
o
t
 
m
o
v
e
,


b
e
c
a
u
s
e
 
i
t
 
n
a
m
e
s
 
t
h
e
 
c
h
u
n
k
 
m
o
s
t
 
r
e
c
e
n
t
l
y
 
*
a
l
l
o
c
a
t
e
d
 
a
s
 
a
 
c
h
u
n
k
*
 
r
a
t
h
e
r
 
t
h
a
n
 
t
h
e


i
n
o
d
e
 
m
o
s
t
 
r
e
c
e
n
t
l
y
 
h
a
n
d
e
d
 
o
u
t
.




-
-
-




#
#
 
T
e
s
t
 
s
t
r
a
t
e
g
y




T
h
r
e
e
 
l
a
y
e
r
s
,
 
a
n
d
 
a
 
f
e
a
t
u
r
e
 
i
s
 
n
o
t
 
d
o
n
e
 
w
i
t
h
o
u
t
 
a
l
l
 
t
h
r
e
e
.




1
.
 
*
*
U
n
i
t
.
*
*
 
A
 
s
m
a
l
l
 
i
n
-
m
e
m
o
r
y
 
t
r
e
e
,
 
h
e
a
d
e
r
 
o
r
 
l
i
s
t
,
 
a
n
d
 
t
h
e
 
e
x
a
c
t
 
m
u
t
a
t
i
o
n
.


2
.
 
*
*
I
m
a
g
e
.
*
*
 
A
 
c
o
p
y
 
o
f
 
a
 
r
e
a
l
 
i
m
a
g
e
,
 
m
o
d
i
f
i
e
d
 
t
h
r
o
u
g
h
 
a
 
r
e
a
l
 
t
r
a
n
s
a
c
t
i
o
n
,
 
a
n
d


 
 
 
t
h
e
 
m
e
t
a
d
a
t
a
 
r
e
a
d
 
b
a
c
k
 
a
f
t
e
r
w
a
r
d
s
.


3
.
 
*
*
R
e
p
a
i
r
.
*
*
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
,
 
a
n
d
 
`
x
f
s
_
d
b
`
 
w
h
e
r
e
 
t
h
e
 
q
u
e
s
t
i
o
n
 
i
s
 
a
b
o
u
t
 
a


 
 
 
f
i
e
l
d
'
s
 
v
a
l
u
e
.
 
 
W
h
e
r
e
 
t
h
e
 
t
o
o
l
s
 
a
r
e
 
n
o
t
 
i
n
s
t
a
l
l
e
d
 
t
h
e
 
t
e
s
t
 
s
k
i
p
s
 
c
l
e
a
n
l
y


 
 
 
r
a
t
h
e
r
 
t
h
a
n
 
f
a
i
l
i
n
g
.




`
x
f
u
s
e
`
'
s
 
o
w
n
 
d
e
c
o
d
e
r
 
i
s
 
n
e
v
e
r
 
t
h
e
 
o
n
l
y
 
o
r
a
c
l
e
.
 
 
A
 
s
e
r
i
a
l
i
z
e
r
 
t
h
a
t
 
w
r
i
t
e
s


e
x
a
c
t
l
y
 
w
h
a
t
 
i
t
s
 
o
w
n
 
p
a
r
s
e
r
 
e
x
p
e
c
t
s
 
c
a
n
 
s
t
i
l
l
 
b
e
 
w
r
o
n
g
,
 
w
h
i
c
h
 
i
s
 
h
o
w
 
a
 
w
r
o
n
g


s
u
p
e
r
b
l
o
c
k
 
o
f
f
s
e
t
 
s
u
r
v
i
v
e
d
 
a
s
 
l
o
n
g
 
a
s
 
i
t
 
d
i
d
.




E
v
e
r
y
 
F
U
S
E
-
l
e
v
e
l
 
t
e
s
t
 
i
n
 
`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
 
m
o
u
n
t
s
 
r
e
a
d
-
w
r
i
t
e
,
 
p
e
r
f
o
r
m
s
 
t
h
e


o
p
e
r
a
t
i
o
n
,
 
u
n
m
o
u
n
t
s
,
 
a
n
d
 
t
h
e
n
 
*
*
m
o
u
n
t
s
 
r
e
a
d
-
o
n
l
y
 
a
g
a
i
n
*
*
 
t
o
 
c
h
e
c
k
 
t
h
e
 
r
e
s
u
l
t
.


R
e
a
d
i
n
g
 
o
n
e
'
s
 
o
w
n
 
w
r
i
t
e
s
 
b
a
c
k
 
t
h
r
o
u
g
h
 
t
h
e
 
s
a
m
e
 
m
o
u
n
t
 
w
o
u
l
d
 
t
e
s
t
 
t
h
e
 
k
e
r
n
e
l
'
s


p
a
g
e
 
c
a
c
h
e
 
r
a
t
h
e
r
 
t
h
a
n
 
t
h
e
 
i
m
a
g
e
,
 
a
n
d
 
a
n
 
e
a
r
l
y
 
v
e
r
s
i
o
n
 
o
f
 
o
n
e
 
o
f
 
t
h
e
s
e
 
t
e
s
t
s


p
a
s
s
e
d
 
w
h
i
l
e
 
t
h
e
 
w
r
i
t
e
 
p
a
t
h
 
w
a
s
 
d
r
o
p
p
i
n
g
 
t
h
e
 
o
f
f
s
e
t
 
w
i
t
h
i
n
 
a
 
b
l
o
c
k
.




T
h
e
 
a
l
l
o
c
a
t
o
r
'
s
 
o
w
n
 
t
e
s
t
s
 
r
u
n
 
a
g
a
i
n
s
t
 
*
*
`
x
f
s
v
4
.
i
m
g
`
 
f
o
r
 
e
v
e
r
y
t
h
i
n
g
 
t
h
a
t
 
m
o
d
i
f
i
e
s


a
n
 
i
m
a
g
e
*
*
,
 
w
h
i
c
h
 
h
a
s
 
n
o
 
c
h
e
c
k
s
u
m
s
 
a
n
d
 
s
h
o
r
t
 
b
-
t
r
e
e
 
h
e
a
d
e
r
s
.
 
 
S
o
 
t
h
e
 
v
e
r
s
i
o
n
 
5


w
r
i
t
e
 
p
a
t
h
 
—
 
5
6
-
b
y
t
e
 
b
l
o
c
k
 
h
e
a
d
e
r
s
,
 
t
h
e
 
o
w
n
e
r
 
a
n
d
 
i
d
e
n
t
i
f
i
e
r
 
i
n
s
i
d
e
 
t
h
e
m
,
 
t
h
e


c
h
e
c
k
s
u
m
 
o
n
 
e
v
e
r
y
 
b
l
o
c
k
 
t
h
i
s
 
c
o
d
e
 
r
e
w
r
i
t
e
s
,
 
a
n
d
 
t
h
e
 
f
r
e
e
 
l
i
s
t
'
s
 
o
w
n
 
h
e
a
d
e
r
 
a
n
d


c
h
e
c
k
s
u
m
 
—
 
w
a
s
 
s
e
p
a
r
a
t
e
l
y
 
u
n
e
x
e
r
c
i
s
e
d
 
a
g
a
i
n
s
t
 
a
 
r
e
a
l
 
i
m
a
g
e
.
 
 
I
t
 
i
s
 
n
o
w
,
 
o
n


`
x
f
s
_
4
k
n
.
i
m
g
`
:
 
t
h
e
 
s
a
m
e
 
s
e
q
u
e
n
c
e
 
t
h
e
 
v
e
r
s
i
o
n
 
4
 
t
e
s
t
 
r
u
n
s
 
(
a
l
l
o
c
a
t
e
,
 
g
i
v
e
 
a
 
r
u
n


b
a
c
k
,
 
t
a
k
e
 
a
 
b
-
t
r
e
e
 
n
o
d
e
 
a
n
d
 
p
u
t
 
i
t
 
b
a
c
k
,
 
c
u
t
 
a
 
h
o
l
e
 
i
n
 
a
 
r
u
n
)
 
w
i
t
h
 
t
h
e


a
c
c
o
u
n
t
i
n
g
 
o
r
a
c
l
e
 
a
s
k
e
d
 
a
f
t
e
r
 
e
a
c
h
 
s
t
e
p
,
 
a
n
d
 
`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
a
s
k
e
d
 
a
t
 
t
h
e
 
e
n
d
.




T
h
a
t
 
t
e
s
t
 
w
a
s
 
c
o
n
f
i
r
m
e
d
 
t
o
 
f
a
i
l
 
w
h
e
n
 
t
h
e
 
c
h
e
c
k
s
u
m
 
p
a
t
h
 
i
s
 
b
r
o
k
e
n
 
o
n
 
p
u
r
p
o
s
e
,


w
h
i
c
h
 
i
s
 
t
h
e
 
o
n
l
y
 
w
a
y
 
t
o
 
k
n
o
w
 
a
 
c
h
e
c
k
 
o
f
 
t
h
a
t
 
k
i
n
d
 
i
s
 
d
o
i
n
g
 
a
n
y
t
h
i
n
g
:
 
d
i
s
a
b
l
i
n
g


`
A
g
f
:
:
u
p
d
a
t
e
_
c
r
c
`
 
m
a
k
e
s
 
i
t
 
f
a
i
l
 
o
n
 
t
h
e
 
g
r
o
u
p
 
h
e
a
d
e
r
,
 
a
n
d
 
l
e
a
v
i
n
g
 
a
 
b
-
t
r
e
e
 
n
o
d
e
'
s


c
h
e
c
k
s
u
m
 
s
t
a
l
e
 
m
a
k
e
s
 
i
t
 
f
a
i
l
 
o
n
 
t
h
e
 
n
o
d
e
.




`
t
e
s
t
s
/
w
r
i
t
e
.
r
s
`
 
c
u
r
r
e
n
t
l
y
 
c
o
v
e
r
s
:
 
o
v
e
r
w
r
i
t
e
 
o
f
 
a
 
b
y
t
e
;
 
o
v
e
r
w
r
i
t
e
 
s
u
r
v
i
v
i
n
g
 
a


r
e
m
o
u
n
t
;
 
p
a
r
t
i
a
l
-
b
l
o
c
k
 
w
r
i
t
e
s
 
w
i
t
h
 
t
h
e
 
s
u
r
r
o
u
n
d
i
n
g
 
b
y
t
e
s
 
c
h
e
c
k
e
d
;
 
w
h
o
l
e
-
b
l
o
c
k


a
n
d
 
u
n
a
l
i
g
n
e
d
 
w
r
i
t
e
s
 
a
t
 
s
e
v
e
n
 
o
f
f
s
e
t
s
;
 
w
r
i
t
e
s
 
t
o
 
f
i
l
e
s
 
o
f
 
s
e
v
e
r
a
l
 
s
h
a
p
e
s
;
 
t
h
e


l
a
s
t
 
b
y
t
e
 
o
f
 
a
 
f
i
l
e
 
w
r
i
t
a
b
l
e
 
a
n
d
 
t
h
e
 
b
y
t
e
 
a
f
t
e
r
 
i
t
 
r
e
f
u
s
e
d
;
 
a
 
w
r
i
t
e
 
p
a
s
t
 
t
h
e
 
e
n
d


g
r
o
w
i
n
g
 
t
h
e
 
f
i
l
e
,
 
v
e
r
i
f
i
e
d
 
t
h
r
o
u
g
h
 
a
 
s
e
c
o
n
d
 
r
e
a
d
-
o
n
l
y
 
m
o
u
n
t
;
 
a
 
w
r
i
t
e
 
p
a
s
t
 
t
h
e


e
n
d
 
l
e
a
v
i
n
g
 
a
 
g
a
p
 
t
h
a
t
 
r
e
a
d
s
 
a
s
 
z
e
r
o
e
s
;
 
t
h
e
 
m
o
d
i
f
i
c
a
t
i
o
n
 
t
i
m
e
 
m
o
v
i
n
g
 
o
n
 
t
h
e


i
m
a
g
e
;
 
a
 
r
e
a
d
-
o
n
l
y
 
m
o
u
n
t
 
r
e
f
u
s
i
n
g
 
a
 
w
r
i
t
e
 
w
i
t
h
o
u
t
 
c
h
a
n
g
i
n
g
 
a
 
b
y
t
e
;
 
a
 
r
e
a
d
-
w
r
i
t
e


m
o
u
n
t
 
o
f
 
a
 
f
e
a
t
u
r
e
-
b
e
a
r
i
n
g
 
i
m
a
g
e
 
b
e
i
n
g
 
r
e
f
u
s
e
d
 
b
y
 
n
a
m
e
 
w
h
i
l
e
 
t
h
e
 
s
a
m
e
 
i
m
a
g
e


s
t
i
l
l
 
m
o
u
n
t
s
 
r
e
a
d
-
o
n
l
y
;
 
a
 
w
r
i
t
e
 
t
o
 
a
 
d
i
r
e
c
t
o
r
y
 
b
e
i
n
g
 
r
e
f
u
s
e
d
;
 
a
n
d


`
x
f
s
_
r
e
p
a
i
r
 
-
n
`
 
b
e
i
n
g
 
h
a
p
p
y
 
a
f
t
e
r
 
t
h
e
 
w
r
i
t
e
s
.




#
#
#
 
I
m
a
g
e
s




T
h
e
 
s
u
i
t
e
 
d
o
e
s
 
n
o
t
 
o
v
e
r
f
i
t
 
t
o
 
t
h
e
 
i
m
a
g
e
s
 
i
t
 
h
a
s
.
 
 
A
 
s
m
a
l
l
 
f
i
l
e
s
y
s
t
e
m
,
 
5
1
2
-
b
y
t
e


a
n
d
 
4
0
9
6
-
b
y
t
e
 
b
l
o
c
k
s
,
 
v
e
r
s
i
o
n
 
4
 
a
n
d
 
v
e
r
s
i
o
n
 
5
 
w
i
t
h
 
C
R
C
,
 
a
 
f
r
a
g
m
e
n
t
e
d
 
i
m
a
g
e
,
 
a
n


i
m
a
g
e
 
w
i
t
h
 
p
o
p
u
l
a
t
e
d
 
f
r
e
e
 
l
i
s
t
s
,
 
a
n
d
 
a
n
 
i
m
a
g
e
 
w
i
t
h
 
m
u
l
t
i
-
l
e
v
e
l
 
f
r
e
e
 
s
p
a
c
e
 
t
r
e
e
s


a
r
e
 
a
l
l
 
w
a
n
t
e
d
.
 
 
W
h
e
r
e
 
a
 
t
e
s
t
 
n
e
e
d
s
 
a
 
p
a
r
t
i
c
u
l
a
r
 
s
h
a
p
e
 
i
t
 
i
s
 
g
e
n
e
r
a
t
e
d
 
w
i
t
h


`
m
k
f
s
.
x
f
s
`
 
a
n
d
 
c
o
n
t
r
o
l
l
e
d
 
f
i
l
e
 
s
y
s
t
e
m
 
o
p
e
r
a
t
i
o
n
s
 
r
a
t
h
e
r
 
t
h
a
n
 
b
y
 
e
d
i
t
i
n
g


m
e
t
a
d
a
t
a
 
—
 
h
a
n
d
-
e
d
i
t
i
n
g
 
i
s
 
f
o
r
 
t
e
s
t
s
 
w
h
o
s
e
 
s
u
b
j
e
c
t
 
*
i
s
*
 
m
a
l
f
o
r
m
e
d
 
m
e
t
a
d
a
t
a
.




-
-
-




#
#
 
D
e
f
i
n
i
t
i
o
n
 
o
f
 
d
o
n
e
 
f
o
r
 
f
u
l
l
 
r
e
a
d
-
w
r
i
t
e
 
s
u
p
p
o
r
t




|
 
I
t
e
m
 
|
 
S
t
a
t
u
s
 
|


|
:
-
-
-
-
-
|
:
-
-
-
-
-
-
-
|


|
 
e
x
i
s
t
i
n
g
 
X
F
S
 
i
m
a
g
e
s
 
s
t
i
l
l
 
m
o
u
n
t
 
r
e
a
d
-
o
n
l
y
 
|
 
d
o
n
e
 
|


|
 
e
x
i
s
t
i
n
g
 
r
e
a
d
 
t
e
s
t
s
 
s
t
i
l
l
 
p
a
s
s
 
|
 
d
o
n
e
 
|


|
 
w
r
i
t
a
b
l
e
 
m
o
u
n
t
 
c
a
n
 
b
e
 
e
x
p
l
i
c
i
t
l
y
 
r
e
q
u
e
s
t
e
d
 
|
 
d
o
n
e
 
|


|
 
u
n
s
u
p
p
o
r
t
e
d
 
X
F
S
 
f
e
a
t
u
r
e
s
 
c
a
u
s
e
 
a
 
r
e
a
d
-
w
r
i
t
e
 
m
o
u
n
t
 
r
e
j
e
c
t
i
o
n
 
|
 
d
o
n
e
 
|


|
 
e
x
i
s
t
i
n
g
 
a
l
l
o
c
a
t
e
d
 
f
i
l
e
 
d
a
t
a
 
c
a
n
 
b
e
 
o
v
e
r
w
r
i
t
t
e
n
 
|
 
d
o
n
e
 
|


|
 
t
h
e
 
a
l
l
o
c
a
t
o
r
'
s
 
w
r
i
t
e
 
p
a
t
h
 
v
e
r
i
f
i
e
d
 
o
n
 
a
 
v
e
r
s
i
o
n
 
5
 
i
m
a
g
e
 
w
i
t
h
 
c
h
e
c
k
s
u
m
s
 
|
 
d
o
n
e
 
|


|
 
a
 
g
r
o
u
p
 
h
e
a
d
e
r
,
 
f
r
e
e
 
l
i
s
t
,
 
i
n
o
d
e
 
h
e
a
d
e
r
 
a
n
d
 
u
s
e
d
-
i
n
o
d
e
 
t
r
e
e
 
c
a
n
 
b
e
 
r
e
a
d
 
a
n
d
 
w
r
i
t
t
e
n
 
|
 
d
o
n
e
 
|


|
 
a
 
f
r
e
e
 
s
p
a
c
e
 
b
t
r
e
e
 
c
a
n
 
b
e
 
w
a
l
k
e
d
,
 
s
e
a
r
c
h
e
d
 
a
n
d
 
m
u
t
a
t
e
d
 
a
t
 
t
h
e
 
l
e
a
f
 
|
 
d
o
n
e
 
|


|
 
b
l
o
c
k
s
 
c
a
n
 
b
e
 
a
l
l
o
c
a
t
e
d
 
a
n
d
 
g
i
v
e
n
 
b
a
c
k
,
 
t
h
r
o
u
g
h
 
a
 
t
r
a
n
s
a
c
t
i
o
n
 
|
 
d
o
n
e
 
|


|
 
a
n
 
e
x
t
e
n
t
 
c
a
n
 
b
e
 
a
d
d
e
d
 
t
o
 
a
 
f
i
l
e
,
 
a
n
d
 
t
h
e
 
f
i
l
e
 
g
r
o
w
n
 
|
 
d
o
n
e
 
|


|
 
a
 
g
a
p
 
r
e
a
d
s
 
a
s
 
z
e
r
o
e
s
 
|
 
d
o
n
e
 
|


|
 
a
n
 
i
n
o
d
e
 
c
a
n
 
b
e
 
a
l
l
o
c
a
t
e
d
 
f
r
o
m
 
a
n
 
e
x
i
s
t
i
n
g
 
c
h
u
n
k
 
|
 
d
o
n
e
 
|


|
 
a
 
n
e
w
 
i
n
o
d
e
 
c
h
u
n
k
 
c
a
n
 
b
e
 
a
l
l
o
c
a
t
e
d
 
i
n
 
a
 
g
r
o
u
p
 
t
h
a
t
 
h
a
s
 
n
o
n
e
 
|
 
d
o
n
e
 
|


|
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
 
c
a
n
 
b
e
 
t
a
k
e
n
 
f
o
r
 
a
 
l
i
v
e
 
b
-
t
r
e
e
 
n
o
d
e
,
 
w
i
t
h
 
t
h
e
 
a
c
c
o
u
n
t
i
n
g
 
X
F
S
 
e
x
p
e
c
t
s
 
|
 
d
o
n
e
 
|


|
 
a
 
m
e
t
a
d
a
t
a
 
b
l
o
c
k
 
t
h
a
t
 
i
s
 
n
o
 
l
o
n
g
e
r
 
n
e
e
d
e
d
 
c
a
n
 
b
e
 
g
i
v
e
n
 
b
a
c
k
,
 
t
o
 
t
h
e
 
l
i
s
t
 
o
r
 
t
o
 
f
r
e
e
 
s
p
a
c
e
 
|
 
d
o
n
e
 
|


|
 
a
 
f
i
l
e
 
c
a
n
 
b
e
 
m
a
d
e
 
s
h
o
r
t
e
r
,
 
g
i
v
i
n
g
 
i
t
s
 
b
l
o
c
k
s
 
b
a
c
k
 
|
 
d
o
n
e
 
|


|
 
f
r
e
e
 
s
p
a
c
e
 
l
e
a
f
 
m
e
r
g
e
 
a
n
d
 
p
a
r
e
n
t
 
r
e
m
o
v
a
l
 
|
 
d
o
n
e
 
|


|
 
f
r
e
e
 
s
p
a
c
e
 
r
o
o
t
 
c
o
l
l
a
p
s
e
 
|
 
n
o
t
 
s
t
a
r
t
e
d
 
|


|
 
a
 
n
e
w
 
i
n
o
d
e
 
c
h
u
n
k
 
c
a
n
 
b
e
 
a
l
l
o
c
a
t
e
d
 
|
 
d
o
n
e
 
|


|
 
a
 
f
i
l
e
 
w
h
o
s
e
 
d
a
t
a
 
f
o
r
k
 
i
s
 
a
 
B
+
t
r
e
e
 
c
a
n
 
b
e
 
w
r
i
t
t
e
n
 
|
 
n
o
t
 
s
t
a
r
t
e
d
 
|


|
 
f
i
l
e
s
 
c
a
n
 
b
e
 
t
r
u
n
c
a
t
e
d
 
a
n
d
 
t
h
e
i
r
 
b
l
o
c
k
s
 
r
e
t
u
r
n
e
d
 
|
 
d
o
n
e
 
|


|
 
a
 
f
i
l
e
'
s
 
e
x
t
e
n
t
s
 
t
h
a
t
 
o
u
t
g
r
o
w
 
i
t
s
 
i
n
o
d
e
 
a
r
e
 
r
e
f
u
s
e
d
 
r
a
t
h
e
r
 
t
h
a
n
 
m
i
s
l
a
i
d
 
|
 
d
o
n
e
 
|


|
 
f
i
l
e
s
 
c
a
n
 
b
e
 
c
r
e
a
t
e
d
,
 
u
n
l
i
n
k
e
d
 
a
n
d
 
r
e
n
a
m
e
d
 
|
 
n
o
t
 
s
t
a
r
t
e
d
 
|


|
 
d
i
r
e
c
t
o
r
i
e
s
 
c
a
n
 
b
e
 
c
r
e
a
t
e
d
 
a
n
d
 
r
e
m
o
v
e
d
 
|
 
n
o
t
 
s
t
a
r
t
e
d
 
|


|
 
t
h
e
 
j
o
u
r
n
a
l
 
w
o
r
k
s
,
 
a
n
d
 
r
e
c
o
v
e
r
y
 
f
r
o
m
 
a
 
t
o
r
n
 
w
r
i
t
e
 
|
 
n
o
t
 
s
t
a
r
t
e
d
 
|


|
 
`
x
f
s
_
r
e
p
a
i
r
`
 
r
e
p
o
r
t
s
 
n
o
 
u
n
e
x
p
e
c
t
e
d
 
c
o
r
r
u
p
t
i
o
n
 
|
 
d
o
n
e
 
f
o
r
 
t
h
e
 
i
m
p
l
e
m
e
n
t
e
d
 
s
u
b
s
e
t
 
|


|
 
n
a
t
i
v
e
 
X
F
S
 
c
a
n
 
m
o
u
n
t
 
a
 
f
i
l
e
 
s
y
s
t
e
m
 
`
x
f
u
s
e
`
 
w
r
o
t
e
 
|
 
p
e
n
d
i
n
g
 
(
n
e
e
d
s
 
r
o
o
t
 
a
n
d
 
a
 
l
o
o
p
 
d
e
v
i
c
e
)
 
|


|
 
l
i
c
e
n
s
i
n
g
 
a
u
d
i
t
 
c
o
n
f
i
r
m
s
 
n
o
 
G
P
L
-
d
e
r
i
v
e
d
 
s
o
u
r
c
e
 
|
 
d
o
n
e
 
|




#
#
 
L
i
c
e
n
s
i
n
g
 
r
e
v
i
e
w




N
o
 
G
P
L
 
s
o
u
r
c
e
 
w
a
s
 
c
o
n
s
u
l
t
e
d
,
 
c
o
p
i
e
d
,
 
t
r
a
n
s
l
a
t
e
d
,
 
o
r
 
a
d
a
p
t
e
d
 
w
h
i
l
e
 
d
o
i
n
g
 
a
n
y
 
o
f


t
h
i
s
.
 
 
T
h
e
 
f
o
r
m
a
t
 
d
e
t
a
i
l
s
 
c
a
m
e
 
f
r
o
m
 
t
h
e
 
p
u
b
l
i
s
h
e
d
 
X
F
S
 
o
n
-
d
i
s
k
 
f
o
r
m
a
t


d
o
c
u
m
e
n
t
a
t
i
o
n
,
 
a
n
d
 
e
v
e
r
y
 
u
n
c
e
r
t
a
i
n
t
y
 
w
a
s
 
s
e
t
t
l
e
d
 
b
y
 
e
x
p
e
r
i
m
e
n
t
 
a
g
a
i
n
s
t
 
i
m
a
g
e
s


p
r
o
d
u
c
e
d
 
b
y
 
`
m
k
f
s
.
x
f
s
`
 
a
n
d
 
r
e
a
d
 
b
a
c
k
 
w
i
t
h
 
`
x
f
s
_
d
b
`
 
—
 
n
o
t
a
b
l
y
:




*
 
w
h
e
r
e
 
t
h
e
 
e
x
t
e
n
t
 
c
o
u
n
t
 
l
i
v
e
s
 
f
o
r
 
a
n
 
i
n
o
d
e
 
t
h
a
t
 
d
o
e
s
 
n
o
t
 
u
s
e
 
6
4
-
b
i
t
 
c
o
u
n
t
s
 
(
a


 
 
3
2
-
b
i
t
 
f
i
e
l
d
 
b
e
s
i
d
e
 
t
h
e
 
a
t
t
r
i
b
u
t
e
 
f
o
r
k
'
s
 
c
o
u
n
t
,
 
n
o
t
 
t
h
e
 
1
6
-
b
i
t
 
o
n
e
 
a
b
o
v
e
 
i
t
)
,


 
 
s
e
t
t
l
e
d
 
b
y
 
c
h
a
n
g
i
n
g
 
t
h
e
 
b
y
t
e
s
 
a
n
d
 
w
a
t
c
h
i
n
g
 
w
h
i
c
h
 
f
i
e
l
d
 
m
o
v
e
d
;


*
 
t
h
a
t
 
a
 
b
i
g
-
t
i
m
e
 
t
i
m
e
s
t
a
m
p
 
i
s
 
a
 
n
a
n
o
s
e
c
o
n
d
 
c
o
u
n
t
 
f
r
o
m
 
1
9
0
1
-
1
2
-
1
3
 
2
0
:
4
5
:
5
2
 
U
T
C
,


 
 
s
e
t
t
l
e
d
 
b
y
 
d
e
c
o
d
i
n
g
 
a
 
r
e
a
l
 
i
n
o
d
e
 
t
w
o
 
w
a
y
s
 
a
n
d
 
c
h
e
c
k
i
n
g
 
w
h
i
c
h
 
m
a
t
c
h
e
d
 
w
h
a
t


 
 
`
x
f
s
_
d
b
`
 
p
r
i
n
t
e
d
;


*
 
t
h
a
t
 
t
h
e
 
v
e
r
s
i
o
n
 
3
 
i
n
o
d
e
 
c
h
e
c
k
s
u
m
 
a
n
d
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
 
c
h
e
c
k
s
u
m
 
a
r
e
 
s
t
o
r
e
d


 
 
l
e
a
s
t
 
s
i
g
n
i
f
i
c
a
n
t
 
b
y
t
e
 
f
i
r
s
t
,
 
a
n
d
 
t
h
a
t
 
t
h
e
 
s
u
p
e
r
b
l
o
c
k
'
s
 
c
o
v
e
r
s
 
o
n
e
 
5
1
2
-
b
y
t
e


 
 
s
e
c
t
o
r
 
w
i
t
h
 
t
h
e
 
c
h
e
c
k
s
u
m
 
f
i
e
l
d
 
i
t
s
e
l
f
 
z
e
r
o
e
d
;


*
 
t
h
a
t
 
t
h
e
 
g
r
o
u
p
 
f
r
e
e
 
l
i
s
t
 
c
a
r
r
i
e
s
 
a
 
h
e
a
d
e
r
 
o
n
 
a
 
v
e
r
s
i
o
n
 
5
 
i
m
a
g
e
 
a
n
d
 
d
o
e
s
 
n
o
t


 
 
o
n
 
a
 
v
e
r
s
i
o
n
 
4
 
o
n
e
,
 
s
e
t
t
l
e
d
 
b
y
 
d
u
m
p
i
n
g
 
t
h
e
 
s
a
m
e
 
s
t
r
u
c
t
u
r
e
 
o
u
t
 
o
f
 
a
 
4
 
K
i
B
 
a
n
d


 
 
a
 
5
1
2
-
b
y
t
e
 
i
m
a
g
e
;


*
 
a
n
d
 
t
h
e
 
t
h
r
e
e
 
i
n
v
a
r
i
a
n
t
s
 
i
n
 
[
M
e
a
s
u
r
e
d
 
i
n
v
a
r
i
a
n
t
s
]
(
#
m
e
a
s
u
r
e
d
-
i
n
v
a
r
i
a
n
t
s
)
,
 
e
a
c
h


 
 
o
f
 
w
h
i
c
h
 
w
a
s
 
c
h
e
c
k
e
d
 
a
g
a
i
n
s
t
 
t
h
e
 
i
m
a
g
e
 
a
n
d
 
a
g
a
i
n
s
t
 
`
x
f
s
_
r
e
p
a
i
r
`
'
s
 
o
w
n
 
o
u
t
p
u
t


 
 
r
a
t
h
e
r
 
t
h
a
n
 
d
e
r
i
v
e
d
 
f
r
o
m
 
a
n
y
 
i
m
p
l
e
m
e
n
t
a
t
i
o
n
.




T
h
e
 
i
n
s
t
r
u
m
e
n
t
 
u
s
e
d
 
f
o
r
 
t
h
o
s
e
 
m
e
a
s
u
r
e
m
e
n
t
s
 
i
s
 
a
 
t
h
r
o
w
a
w
a
y
 
s
c
r
i
p
t
,
 
n
o
t
 
p
a
r
t
 
o
f


t
h
e
 
r
e
p
o
s
i
t
o
r
y
,
 
a
n
d
 
s
h
a
r
e
s
 
n
o
 
c
o
d
e
 
w
i
t
h
 
`
x
f
u
s
e
`
 
o
r
 
w
i
t
h
 
`
x
f
s
p
r
o
g
s
`
.




T
h
e
 
t
w
o
 
i
n
o
d
e
s
 
e
m
b
e
d
d
e
d
 
i
n
 
`
s
r
c
/
l
i
b
x
f
u
s
e
/
i
n
o
d
e
.
r
s
`
'
s
 
t
e
s
t
s
 
a
s
 
b
y
t
e
 
s
t
r
i
n
g
s
 
c
a
m
e


f
r
o
m
 
t
h
e
 
g
o
l
d
e
n
 
i
m
a
g
e
s
 
b
y
 
r
e
a
d
i
n
g
 
t
h
e
m
 
w
i
t
h
 
a
 
s
c
r
i
p
t
;
 
t
h
e
y
 
a
r
e
 
t
e
s
t
 
d
a
t
a
,
 
a
n
d


t
h
e
y
 
a
r
e
 
t
h
e
r
e
 
s
o
 
t
h
a
t
 
a
 
c
h
a
n
g
e
 
t
o
 
t
h
e
 
f
i
e
l
d
 
l
a
y
o
u
t
 
s
h
o
w
s
 
u
p
 
a
s
 
a
 
f
a
i
l
i
n
g
 
t
e
s
t


r
a
t
h
e
r
 
t
h
a
n
 
a
s
 
a
 
w
r
o
n
g
 
w
r
i
t
e
.




S
e
e
 
[
`
l
i
c
e
n
s
i
n
g
.
m
d
`
]
(
l
i
c
e
n
s
i
n
g
.
m
d
)
 
f
o
r
 
t
h
e
 
d
e
p
e
n
d
e
n
c
y
 
a
u
d
i
t
.

