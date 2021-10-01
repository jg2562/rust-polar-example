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

df = from_rust(create_df())

update_df = from_rust(super_advanced_calculation(to_rust(df)))
breakpoint()
