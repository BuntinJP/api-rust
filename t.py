def fit(cls, x, y, deg, domain=None, rcond=None, full=False, w=None,
        window=None, symbol='x'):
    if domain is None:
        domain = pu.getdomain(x)
    elif type(domain) is list and len(domain) == 0:
        domain = cls.domain

    if window is None:
        window = cls.window

    xnew = pu.mapdomain(x, domain, window)
    res = cls._fit(xnew, y, deg, w=w, rcond=rcond, full=full)
    if full:
        [coef, status] = res
        return (
            cls(coef, domain=domain, window=window, symbol=symbol), status
        )
    else:
        coef = res
        return cls(coef, domain=domain, window=window, symbol=symbol)
