// Copyright (C) 2022 Lily Lyons
//
// This file is part of libfmod.
//
// libfmod is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// libfmod is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libfmod.  If not, see <http://www.gnu.org/licenses/>.

use crate::bindable_enum;

bindable_enum!(
    ChannelControlCallbackType,
    End,
    VirtualVoice,
    SyncPoint,
    Occlusion,
    Max
);
bindable_enum!(LoadMemoryMode, Memory, MemoryPoint);
bindable_enum!(LoadingState, Unloading, Unloaded, Loading, Loaded, Error);
bindable_enum!(
    ParameterType,
    GameControlled,
    AutomaticDistance,
    AutomaticEventConeAngle,
    AutomaticEventOrientation,
    AutomaticDirection,
    AutomaticElevation,
    AutomaticListenerOrientation,
    AutomaticSpeed,
    AutomaticSpeedAbsolute,
    AutomaticDistanceNormalized,
    Max
);

pub fn bind_enums(module: impl magnus::Module) -> Result<(), magnus::Error> {
    ChannelControlCallbackType::bind(module)?;
    LoadMemoryMode::bind(module)?;
    LoadingState::bind(module)?;
    ParameterType::bind(module)?;

    Ok(())
}
