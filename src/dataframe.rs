// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Error;

use super::rel::*;
use super::exec::*;

#[derive(Debug)]
pub enum DataFrameError {
    TBD,
    NotImplemented
}

impl From<ExecutionError> for DataFrameError {
    fn from(_e: ExecutionError) -> Self {
        //TODO:
        DataFrameError::TBD
    }
}

impl From<Error> for DataFrameError {
    fn from(_e: Error) -> Self {
        //TODO:
        DataFrameError::TBD
    }
}

/// DataFrame is an abstraction of a distributed query plan
pub trait DataFrame {

    /// Change the number of partitions
    fn repartition(&self, n: u32) -> Result<Box<DataFrame>,DataFrameError>;

    /// Projection
    fn select(&self, expr: Vec<Rex>) -> Result<Box<DataFrame>,DataFrameError>;

    /// Selection
    fn filter(&self, expr: Rex) -> Result<Box<DataFrame>,DataFrameError>;

    /// Write to CSV ...  will support other formats in the future
    fn write(&self, filename: &str) -> Result<(),DataFrameError>;

    /// Return an expression representing the specified column
    fn col(&self, column_name: &str) -> Result<Rex,DataFrameError>;
}
