
export interface CategorysType {
  id: number;
  title: string;
}
export interface ClassificationType {
  id: number;
  title: string;
}

export interface DappType {
  ID: number;
  image: string;
  title: string;
  introduction: string;
  url: string;
  twitter: string;
  discord: string;
  telegram: string;
  value?: string
}

export interface BlogType {
  ID: number;
  bloguserId: string;
  title: string;
  keywords: string;
  image: string;
  introduction: string;
  content: string;
  releaseTime: string;
}

export interface BlogDetailType {
  detail: BlogDetail;
  previous: PreviousType | null;
  next: PreviousType | null;
}

interface PreviousType {
  ID: number;
  title: string;
}

interface BlogDetail {
  ID: number;
  avatar: string;
  name: string;
  bloguserId: string;
  title: string;
  keywords: string;
  image: string;
  introduction: string;
  content: string;
  releaseTime: string;
}

export interface ConfigType {
  Finality: string;
  Second: string;
  Transactions: string;
  Users: string;
  image: string;
  privacy: string;
  discord: string;
  medium: string;
  twitter: string;
  tme: string;
  youtube: string;
  github: string;
  service: string;
}

export interface UserInfoType {
  ID: number;
  uuid: string;
  userName: string;
  nickName: string;
  headerImg: string;
  email: string;
  enable: number;
  token: string;
}

export interface EventListItemType {
  ID: number;
  title: string;
  image: string;
  introduction: string;
  endTime: string;
}

export interface EventDetailType {
  ID: number;
  title: string;
  image: string;
  content: string;
  location: string;
  startTime: string;
  endTime: string;
}

export interface MainListType {
  main: string[];
  money: string[];
}
export interface AmountListType {
  value: string;
  label: string;
}

export interface IndexInfoType {
  ID: number;
  type: string;
  url: string;
  keywords: string;
  title: string;
  image: string;
  introduction: string;
}

export interface NetworkType extends IndexInfoType {
  content: string;
}

export interface GrantCateType {
  ID: number;
  title: string;
  keywords: string;
  introduction: string;
  contentjson: string;
  arr: string[]
}

export interface ForumcategoryChildType {
  id: number;
  flag: string;
  introduction: string;
  color: string;
  forumcategory_id: number;
  level: string;
}

export interface ForumcategoryType {
  ID: number;
  color: string;
  flag: string;
  introduction: string;
}

export interface ForumListItemType {
  ID: number;
  userId: number;
  forumcategoryId: number;
  title: string;
  like: number;
  replies: number;
  content: string;
  switch: string;
  cswitch: string;
  releasetime: string;
  userName: string;
  headerImg: string;
  flag: Flag[];
  last: Last;
}

interface Last {
  userId: number;
  userName: string;
  headerImg: string;
  CreatedAt: string;
}

interface Flag {
  ID: number;
  color: string;
  flag: string;
}

export interface CommunityDetailType {
  info: DetailInfo;
  flag: DetailFlag[];
}

interface DetailFlag {
  ID: number;
  color: string;
  flag: string;
}

interface DetailInfo {
  ID: number;
  userId: number;
  forumcategoryId: number;
  title: string;
  like: number;
  replies: number;
  content: string;
  switch: string;
  releasetime: string;
  userName: string;
  headerImg: string;
  is_like: string;
}

export interface AllChainType {
  ID: number;
  title: string;
  introductionJson: string;
  introduction: any;
  image: string;
  type: string;
  keywords: string;
  arr: string[]
}

export interface CommentItemType {
  ID: number;
  userId: number;
  forumId: number;
  like: number;
  from_user: string;
  from_avatar: string;
  content: string;
  replies: number;
  is_my: number;
  is_like: number;
  CreatedAt: string;
  list: CommentChildItemType[];
  isOpen: boolean;
}

interface CommentChildItemType {
  ID: number;
  forum_id: number;
  like: number;
  from_user_id: number;
  from_user: string;
  from_avatar: string;
  forumcomment_id: number;
  to_user_id: number;
  to_user: string;
  content: string;
  CreatedAt: string;
  is_my: number;
  is_like: number;
}

export interface ActivityType {
  ID: number;
  userId: number;
  forumId: number;
  content: string;
  userName: string;
  headerImg: string;
  forumcategoryId: number;
  title: string;
  color: string;
  flag: string;
  CreatedAt: string;
}

interface Data {
  userName: string;
  headerImg: string;
  userId: number;
  title: string;
  content: string;
  color: string;
  flag: string;
  forumId: number;
  forumcategoryId: number;
  CreatedAt: string;
}
export interface LikesItemType {
  ID: number;
  CreatedAt: string;
  UpdatedAt: string;
  userId: number;
  type: string;
  lid: number;
  like: number;
  notifications: number;
  Data: Data;
}

interface LikesDataType {
  forum_id: number;
  user_id: number;
  title: string;
  content: string;
  username: string;
  avatar: string;
  createtime: number;
  forumcategory_id: number;
  flag: string;
  color: string;
}

export interface NotificationsType {
  userId: number;
  forumId: number;
  notifications: number;
  userName: string;
  forumcategoryId: number;
  title: string;
  CreatedAt: string;
}

export interface ResponsesType {
  ID: number;
  userId: number;
  forumId: number;
  content: string;
  userName: string;
  headerImg: string;
  forumcategoryId: number;
  title: string;
  color: string;
  flag: string;
  CreatedAt: string;
}

export interface LikesReceivedType {
  ID: number;
  userId: number;
  forumId: number;
  content: string;
  userName: string;
  headerImg: string;
  forumcategoryId: number;
  title: string;
  color: string;
  flag: string;
  CreatedAt: string;
}

export interface ParamsLocale {
  params: {
    locale: string
    id?: string | number
    arg?: string[]
    menuValue?: string[]
  }
}

export interface MetaDataType {
  ID: number;
  title: string;
  keywords: string;
  image: string;
  introduction: string;
}