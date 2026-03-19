export function render_meet(node, domain, room, user, isModerator) {
    const options = {
        roomName: room,
        height: 700,
        parentNode: node,
        userInfo: {
            displayName: user,
        },
        configOverwrite: {
            disableInviteFunctions: true,
            prejoinPageEnabled: false,
            defaultLanguage: 'es',
            starWithAudioMuted: true,
            starWithVideoMuted: true,
        },
        interfaceConfigOverwrite: {
            HIDE_INVITE_MORE_HEADER: true,
            TOOLBAR_BUTTONS: [
                'microphone', 'camera', 'closedcaptions', 'desktop', 'fullscreen',
                'fodeviceselection', 'hangup', 'profile', 'chat', 'recording',
                'sharedvideo', 'settings', 'raisehand',
                'videoquality', 'filmstrip', 'feedback', 'stats', 'shortcuts',
                'tileview', 'videobackgroundblur', 'download', 'help', 'mute-everyone'
            ]
        },
    };
    const api = new JitsiMeetExternalAPI(domain, options);
}