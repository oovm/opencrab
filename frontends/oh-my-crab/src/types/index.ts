export type UserRole = 'Admin' | 'User' | 'Guest';

export interface User {
  id: string;
  username: string;
  displayName?: string;
  role: UserRole;
  email?: string;
  createdAt: string;
  updatedAt: string;
  isActive: boolean;
}

export interface Conversation {
  id: string;
  userId: string;
  title: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
  isArchived: boolean;
}

export interface Message {
  id: string;
  conversationId: string;
  userId: string;
  role: string;
  content: string;
  createdAt: string;
  metadata?: string;
}

export interface AppSettings {
  id: string;
  userId: string;
  theme: string;
  language: string;
  apiEndpoint?: string;
  apiKey?: string;
  settingsJson?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateUserRequest {
  username: string;
  displayName?: string;
  role: UserRole;
  email?: string;
}

export interface UpdateUserRequest {
  displayName?: string;
  email?: string;
  isActive?: boolean;
}

export interface CreateConversationRequest {
  title: string;
  description?: string;
}

export interface UpdateConversationRequest {
  title?: string;
  description?: string;
  isArchived?: boolean;
}

export interface CreateMessageRequest {
  role: string;
  content: string;
  metadata?: string;
}
