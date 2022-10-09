# checkpoint-rs
A simple CLI to connect to a Bitcoin-based RPC and produce an easy Checkpoints list.

## Usage

The program has all defaults, and thus it's even possible to run with a plain `./checkpoint-rs`, however, it's very unlikely your RPC daemon is configured this way, so you can configure individual RPC properties as necessary.

```bash
./checkpoint-rs --host=<your_host> --user=<user> --pass=<pass> --start=<height> --interval=<height>

# For example, a zero-arg run of the program would be equal to:

./checkpoint-rs --host=http://localhost:8332 --user=user --pass=pass --start=0 --interval=1
```

For additional help, run the program with a `-h` or `--help` flag!

The results of a successful checkpointing run should look something along the lines of:
```c++
{14120, uint256S("0000000000000679fcdb41979af16aabb1c31f3b10f7a3c980659ea135c1bfb4")},
{14121, uint256S("00000000000005fa7cf85faf78fe93e8460915d94612626a9118b111f50ef56f")},
{14122, uint256S("00000000000004da08b2afff5ea5b58404b40774a041e4943eb002b05d3689b5")},
{14123, uint256S("000000000000027ba01b0e941de5a25445de26e7482ab775f5f2582d328e49b5")},
{14124, uint256S("000000000000010473f79c41323a9cb3d4cc393f7de675a8f39679ea072863b5")},
{14125, uint256S("00000000000004e187babe698aa5b7bca5af0698003a45e7ed1e1bbf4716b512")},
Checkpoints done!
```

Which fits perfectly snug into most Bitcoin-based C++ chainparams.
