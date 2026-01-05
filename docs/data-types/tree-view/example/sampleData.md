import { uuid } from "uuidv4";

export interface TreeItemData {
  id: string;
  name: string;
  children: TreeItemData[];
}
export const sampleFromStackOverflowQuestion: TreeItemData[] = [
  {
    id: uuid(),
    name: "English",
    children: [
      {
        id: uuid(),
        name: "Spring",
        children: []
      }
    ]
  },
  {
    id: uuid(),
    name: "Italian",
    children: [
      {
        id: uuid(),
        name: "Level A",
        children: []
      }
    ]
  }
];

export const seasons: TreeItemData[] = [
  {
    id: uuid(),
    name: "Seasons",
    children: [
      {
        id: uuid(),
        name: "Summer",
        children: [
          {
            id: uuid(),
            name: "June"
          },
          {
            id: uuid(),
            name: "July"
          },
          {
            id: uuid(),
            name: "August"
          }
        ]
      },
      {
        id: uuid(),
        name: "Fall",
        children: [
          {
            id: uuid(),
            name: "September"
          },
          {
            id: uuid(),
            name: "October"
          },
          {
            id: uuid(),
            name: "November"
          }
        ]
      },
      {
        id: uuid(),
        name: "Winter",
        children: [
          {
            id: uuid(),
            name: "December"
          },
          {
            id: uuid(),
            name: "January"
          },
          {
            id: uuid(),
            name: "February"
          }
        ]
      },
      {
        id: uuid(),
        name: "Spring",
        children: [
          {
            id: uuid(),
            name: "March"
          },
          {
            id: uuid(),
            name: "April"
          },
          {
            id: uuid(),
            name: "May"
          }
        ]
      }
    ]
  }
];
