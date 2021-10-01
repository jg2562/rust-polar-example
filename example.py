from tapki_zero import *
import pyarrow
import polars

def from_rust(raw_batches):
    table = pyarrow.Table.from_batches(raw_batches)
    df = polars.from_arrow(table)
    return df

def to_rust(df):
    table = df.to_arrow()
    batches = table.to_batches()
    return batches

# Create data from in rust and receive it in python
df = from_rust(create_df())

# Do some quick and easy work in python
print(df.mean())


# Do a super complicated calculation that can only be done in rust
calced_df = from_rust(super_advanced_calculation(to_rust(df)))

# Check out results in python
print(calced_df)
