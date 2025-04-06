# JellyFish
$$
S(t) = \{(X(x,y,t), Y(x,y,t)) \mid x \in [0,199], y \in [0,199]\}
$$

Where:

$$
\begin{align*}
k(x) &= \frac{x}{8} - 12.5 \\
e(y) &= \frac{y}{8} - 12.5 \\
o(x,y) &= \frac{\text{mag}(k(x), e(y))^2}{169} \\
d(x,y) &= 0.5 + 5\cos(o(x,y)) \\
\text{mag}(k, e) &= \sqrt{k^2 + e^2} \\
\\
X(x,y,t) &= x + d(x,y) \cdot k(x) \cdot \sin(2d(x,y) + o(x,y) + t) + e(y) \cdot \cos(e(y) + t) + 100 \\
Y(x,y,t) &= o(x,y) \cdot 135 - \frac{y}{4} - 6d(x,y) \cdot \cos(3d(x,y) + 9o(x,y) + t) + 125
\end{align*}
$$

# Nudibranch

$$
\begin{align*}
a(x, y) &= \text{point}(X, Y) \\
\text{where:} \\
k &= \frac{x}{8} - 12 \\
e &= \frac{y}{13} - 14 \\
o &= \frac{\sqrt{k^2 + e^2}}{2} \\
d &= 5 \cdot \cos(o) \\
q &= \frac{x}{2} + 10 + \frac{1}{k} + k \cdot \cos(e) \cdot \sin(8d - t) \\
c &= \frac{d}{3} + \frac{t}{8} \\
X &= q \cdot \sin(c) + \sin(2d + t) \cdot k + 200 \\
Y &= \frac{1}{2} \cdot \left(\frac{y}{4} + 5o^2 + q \cdot \cos(3c)\right) \cdot \cos(c) + 200 \\
\end{align*}
$$

**Animation**
$$
\begin{align*}
&t_n = t_{n-1} + \frac{\pi}{60} \\
&\forall i \in \{20000, \ldots, 1\}: \\
&\quad \text{Render point at } a\left(i \bmod 200, \left\lfloor\frac{i}{200}\right\rfloor\right) \\
\end{align*}
$$

# Heartbeat
TODO

# PlanetaryTimer
TODO