interface Category {
  id: number;
  name: string;
}
interface Tabbar extends Category {
  foods: Collection[];
}
