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
    FMOD_CHANNELCONTROL_CALLBACK_TYPE,
    force_int FMOD_CHANNELCONTROL_CALLBACK_FORCEINT;
    End,
    FMOD_CHANNELCONTROL_CALLBACK_END,
    VirtualVoice,
    FMOD_CHANNELCONTROL_CALLBACK_VIRTUALVOICE,
    SyncPoint,
    FMOD_CHANNELCONTROL_CALLBACK_SYNCPOINT,
    Occlusion,
    FMOD_CHANNELCONTROL_CALLBACK_OCCLUSION,
    Max,
    FMOD_CHANNELCONTROL_CALLBACK_MAX
);

bindable_enum!(
    LoadMemoryMode,
    FMOD_STUDIO_LOAD_MEMORY_MODE,
    force_int FMOD_STUDIO_LOAD_MEMORY_FORCEINT;
    Memory,
    FMOD_STUDIO_LOAD_MEMORY,
    MemoryPoint,
    FMOD_STUDIO_LOAD_MEMORY_POINT
);

bindable_enum!(
    LoadingState,
    FMOD_STUDIO_LOADING_STATE,
    force_int FMOD_STUDIO_LOADING_STATE_FORCEINT;
    Unloading,
    FMOD_STUDIO_LOADING_STATE_UNLOADING,
    Unloaded,
    FMOD_STUDIO_LOADING_STATE_UNLOADED,
    Loading,
    FMOD_STUDIO_LOADING_STATE_LOADING,
    Loaded,
    FMOD_STUDIO_LOADING_STATE_LOADED,
    Error,
    FMOD_STUDIO_LOADING_STATE_ERROR
);

bindable_enum!(
    ParameterType,
    FMOD_STUDIO_PARAMETER_TYPE,
    force_int FMOD_STUDIO_PARAMETER_FORCEINT;
    GameControlled,
    FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
    AutomaticDistance,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    AutomaticEventConeAngle,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE,
    AutomaticEventOrientation,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION,
    AutomaticDirection,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION,
    AutomaticElevation,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION,
    AutomaticListenerOrientation,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION,
    AutomaticSpeed,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED,
    AutomaticSpeedAbsolute,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE,
    AutomaticDistanceNormalized,
    FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED,
    Max,
    FMOD_STUDIO_PARAMETER_MAX
);

bindable_enum!(
    UserPropertyType,
    FMOD_STUDIO_USER_PROPERTY_TYPE,
    force_int FMOD_STUDIO_USER_PROPERTY_TYPE_FORCEINT;
    Integer,
    FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER,
    Boolean,
    FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN,
    Float,
    FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT,
    String,
    FMOD_STUDIO_USER_PROPERTY_TYPE_STRING
);

bindable_enum!(
    EventProperty,
    FMOD_STUDIO_EVENT_PROPERTY,
    force_int FMOD_STUDIO_EVENT_PROPERTY_FORCEINT;
    ChannelPriority,
    FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    ScheduleDelay,
    FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
    ScheduleLookahead,
    FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    MinimumDistance,
    FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    MaximumDistance,
    FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    Cooldown,
    FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
    Max,
    FMOD_STUDIO_EVENT_PROPERTY_MAX
);

bindable_enum!(
    StopMode,
    FMOD_STUDIO_STOP_MODE,
    force_int FMOD_STUDIO_STOP_FORCEINT;
    AllowFadeout,
    FMOD_STUDIO_STOP_ALLOWFADEOUT,
    Immediate,
    FMOD_STUDIO_STOP_IMMEDIATE
);

bindable_enum!(
    PlaybackState,
    FMOD_STUDIO_PLAYBACK_STATE,
    force_int FMOD_STUDIO_PLAYBACK_FORCEINT;
    Playing,
    FMOD_STUDIO_PLAYBACK_PLAYING,
    Sustaining,
    FMOD_STUDIO_PLAYBACK_SUSTAINING,
    Stopped,
    FMOD_STUDIO_PLAYBACK_STOPPED,
    Starting,
    FMOD_STUDIO_PLAYBACK_STARTING,
    Stopping,
    FMOD_STUDIO_PLAYBACK_STOPPING
);

bindable_enum!(
    InstanceType,
    FMOD_STUDIO_INSTANCETYPE,
    force_int FMOD_STUDIO_INSTANCETYPE_FORCEINT;
    None,
    FMOD_STUDIO_INSTANCETYPE_NONE,
    System,
    FMOD_STUDIO_INSTANCETYPE_SYSTEM,
    EventDescription,
    FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION,
    EventInstance,
    FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE,
    ParameterInstance,
    FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE,
    Bus,
    FMOD_STUDIO_INSTANCETYPE_BUS,
    Vca,
    FMOD_STUDIO_INSTANCETYPE_VCA,
    Bank,
    FMOD_STUDIO_INSTANCETYPE_BANK,
    CommandReplay,
    FMOD_STUDIO_INSTANCETYPE_BANK
);

pub fn bind_enums(module: impl magnus::Module) -> Result<(), magnus::Error> {
    ChannelControlCallbackType::bind(module)?;
    LoadMemoryMode::bind(module)?;
    LoadingState::bind(module)?;
    ParameterType::bind(module)?;
    UserPropertyType::bind(module)?;
    EventProperty::bind(module)?;
    StopMode::bind(module)?;
    PlaybackState::bind(module)?;
    InstanceType::bind(module)?;

    Ok(())
}
