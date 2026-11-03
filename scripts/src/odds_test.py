from odds import am_to_dec


def test_am_to_dec_positive():
    assert am_to_dec(100) == 2.0
    assert am_to_dec(150) == 2.5
    assert am_to_dec(200) == 3.0
    assert am_to_dec(250) == 3.5
    assert am_to_dec(1000) == 11.0


def test_am_to_dec_negative():
    assert am_to_dec(-100) == 2.0
    assert am_to_dec(-200) == 1.5
    assert am_to_dec(-400) == 1.25
    assert abs(am_to_dec(-110) - 1.9090909) < 1e-6
    assert abs(am_to_dec(-150) - 1.6666667) < 1e-6


def test_am_to_dec_even_money():
    assert am_to_dec(100) == am_to_dec(-100) == 2.0


def test_am_to_dec_always_greater_than_one():
    for odds in [-10000, -500, -110, -100, 100, 110, 500, 10000]:
        assert am_to_dec(odds) > 1.0


def test_am_to_dec_monotonic():
    # bigger positive odds -> bigger payout
    assert am_to_dec(100) < am_to_dec(200) < am_to_dec(500)
    # more negative odds -> smaller payout
    assert am_to_dec(-500) < am_to_dec(-200) < am_to_dec(-110)


def test_am_to_dec_float_input():
    assert am_to_dec(150.0) == 2.5
    assert am_to_dec(-200.0) == 1.5


def test_am_to_dec_zero_raises():
    try:
        am_to_dec(0)
        assert False, "expected ValueError"
    except ValueError:
        pass
