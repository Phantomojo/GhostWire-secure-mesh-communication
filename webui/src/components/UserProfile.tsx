import React from 'react';

interface UserProfileProps {
  user: {
    id: string;
    name: string;
    avatar: string;
    status: string;
    isOnline: boolean;
  };
}

const UserProfile: React.FC<UserProfileProps> = ({ user }) => {
  return (
    <div className="user-profile">
      <div className="user-avatar">
        <span className="avatar-emoji">{user.avatar}</span>
        {user.isOnline && <div className="online-indicator"></div>}
      </div>
      <div className="user-info">
        <h3>{user.name}</h3>
        <p>{user.status}</p>
      </div>
    </div>
  );
};

export default UserProfile; 