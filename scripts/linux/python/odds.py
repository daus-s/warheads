def am_to_dec(n):
    if n == 0:
      raise ValueError("american odds cannot equal 0 exactly")
    elif n < 0:
        return 1 - 100 / n
    else:
        return 1 + n / 100
