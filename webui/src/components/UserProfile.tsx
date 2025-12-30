import React, { useState } from 'react';
import { HiOutlineFingerPrint, HiOutlinePencilSquare, HiOutlineCheck } from 'react-icons/hi2';

interface UserProfileProps {
  user: {
    id: string;
    did: string;
    name: string;
    nickname: string;
    bio: string;
    avatar: string;
    status: string;
    isOnline: boolean;
  };
  onUpdateMetadata?: (metadata: { nickname?: string; bio?: string }) => void;
}

const UserProfile: React.FC<UserProfileProps> = ({ user, onUpdateMetadata }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [nickname, setNickname] = useState(user.nickname);
  const [bio, setBio] = useState(user.bio);

  const handleSave = () => {
    if (onUpdateMetadata) {
      onUpdateMetadata({ nickname, bio });
    }
    setIsEditing(false);
  };

  return (
    <div className="user-profile bg-slate-900/40 p-6 rounded-3xl border border-white/5 backdrop-blur-md">
      <div className="flex items-start justify-between mb-6">
        <div className="user-avatar relative">
          <span className="avatar-emoji text-4xl">{user.avatar}</span>
          {user.isOnline && (
            <div className="absolute -bottom-1 -right-1 w-4 h-4 bg-emerald-500 rounded-full border-4 border-slate-900 animate-pulse"></div>
          )}
        </div>
        <button
          onClick={() => isEditing ? handleSave() : setIsEditing(true)}
          className={`p-2 rounded-xl transition-all ${isEditing ? 'bg-emerald-500/20 text-emerald-400' : 'bg-white/5 text-slate-400 hover:text-white'}`}
        >
          {isEditing ? <HiOutlineCheck className="text-xl" /> : <HiOutlinePencilSquare className="text-xl" />}
        </button>
      </div>

      <div className="user-info space-y-4">
        {isEditing ? (
          <div className="space-y-3">
            <input
              type="text"
              value={nickname}
              onChange={(e) => setNickname(e.target.value)}
              className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-2 text-sm text-white font-black"
              placeholder="Nickname"
            />
            <textarea
              value={bio}
              onChange={(e) => setBio(e.target.value)}
              className="w-full bg-slate-950 border border-white/10 rounded-xl px-4 py-2 text-xs text-slate-300 h-20 resize-none"
              placeholder="Bio"
            />
          </div>
        ) : (
          <div>
            <h3 className="text-xl font-black text-white tracking-tight">{user.nickname || user.name}</h3>
            <p className="text-xs text-slate-400 italic mt-1">{user.bio || 'Sovereign GhostWire Node'}</p>
          </div>
        )}

        <div className="pt-4 border-t border-white/5">
          <div className="flex items-center gap-2 text-[10px] font-black text-indigo-400 uppercase tracking-widest mb-1">
            <HiOutlineFingerPrint />
            Sovereign_DID
          </div>
          <div className="text-[9px] font-mono text-slate-500 break-all bg-slate-950/50 p-2 rounded-lg border border-white/5">
            {user.did || `did:ghost:${user.id.slice(0, 16)}...`}
          </div>
        </div>
      </div>
    </div>
  );
};

export default UserProfile;