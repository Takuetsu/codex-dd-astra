"""Tiny nonexecutable Windows EXE/PDB fixture for testing runtime_package.bzl only."""

def _fake_runtime_binary_impl(ctx):
    exe = ctx.actions.declare_file(ctx.label.name + ".exe")
    ctx.actions.write(
        output = exe,
        content = "CodexDD synthetic nonrelease executable fixture\n",
        is_executable = True,
    )
    symbol_outputs = []
    if ctx.attr.with_pdb:
        pdb = ctx.actions.declare_file(ctx.label.name + ".pdb")
        ctx.actions.write(
            output = pdb,
            content = "CodexDD synthetic nonrelease symbol fixture\n",
        )
        symbol_outputs.append(pdb)
    return [
        DefaultInfo(executable = exe, files = depset([exe] + symbol_outputs)),
        OutputGroupInfo(pdb_file = depset(symbol_outputs)),
    ]

fake_runtime_binary = rule(
    implementation = _fake_runtime_binary_impl,
    executable = True,
    attrs = {"with_pdb": attr.bool(default = True)},
)
