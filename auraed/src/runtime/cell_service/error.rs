/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                +--------------------------------------------+              *
 *                |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |              *
 *                |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |              *
 *                |  ███████║██║   ██║██████╔╝███████║█████╗   |              *
 *                |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |              *
 *                |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |              *
 *                |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |              *
 *                +--------------------------------------------+              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * -------------------------------------------------------------------------- *
 *                                                                            *
 *   Licensed under the Apache License, Version 2.0 (the "License");          *
 *   you may not use this file except in compliance with the License.         *
 *   You may obtain a copy of the License at                                  *
 *                                                                            *
 *       http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                            *
 *   Unless required by applicable law or agreed to in writing, software      *
 *   distributed under the License is distributed on an "AS IS" BASIS,        *
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *   See the License for the specific language governing permissions and      *
 *   limitations under the License.                                           *
 *                                                                            *
\* -------------------------------------------------------------------------- */

use super::{cells::CellsError, executables::ExecutablesError};
use thiserror::Error;
use tonic::Status;
use tracing::error;

pub(crate) type Result<T> = std::result::Result<T, CellsServiceError>;

#[derive(Debug, Error)]
pub(crate) enum CellsServiceError {
    #[error(transparent)]
    CellsError(#[from] CellsError),
    #[error(transparent)]
    ExecutablesError(#[from] ExecutablesError),
}

impl From<CellsServiceError> for Status {
    fn from(err: CellsServiceError) -> Self {
        let msg = err.to_string();
        error!("{msg}");
        match err {
            CellsServiceError::CellsError(e) => match e {
                CellsError::CellExists { .. } => Status::already_exists(msg),
                CellsError::CellNotFound { .. } => Status::not_found(msg),
                CellsError::FailedToAllocateCell { .. }
                | CellsError::AbortedAllocateCell { .. }
                | CellsError::FailedToKillCellChildren { .. }
                | CellsError::FailedToFreeCell { .. }
                | CellsError::CgroupIsNotACell { .. }
                | CellsError::CgroupNotFound { .. } => Status::internal(msg),
                CellsError::CellNotAllocated { cell_name } => {
                    CellsServiceError::CellsError(CellsError::CellNotFound {
                        cell_name,
                    })
                    .into()
                }
            },
            CellsServiceError::ExecutablesError(e) => match e {
                ExecutablesError::ExecutableExists { .. } => {
                    Status::already_exists(msg)
                }
                ExecutablesError::ExecutableNotFound { .. } => {
                    Status::not_found(msg)
                }
                ExecutablesError::FailedToStartExecutable { .. }
                | ExecutablesError::FailedToStopExecutable { .. } => {
                    Status::internal(msg)
                }
            },
        }
    }
}
