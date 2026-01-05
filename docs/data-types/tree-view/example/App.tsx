import * as React from "react";

import TreeView from "@material-ui/lab/TreeView";
import TreeItem from "@material-ui/lab/TreeItem";
import ExpandMoreIcon from "@material-ui/icons/ExpandMore";
import ChevronRightIcon from "@material-ui/icons/ChevronRight";
import {
  sampleFromStackOverflowQuestion,
  seasons,
  TreeItemData,
} from "./sampleData";

const getTreeItemsFromData = (treeItems: TreeItemData[]) => {
  return treeItems.map((treeItemData) => {
    let children = undefined;
    if (treeItemData.children && treeItemData.children.length > 0) {
      children = getTreeItemsFromData(treeItemData.children);
    }
    return (
      <TreeItem
        key={treeItemData.id}
        nodeId={treeItemData.id}
        label={treeItemData.name}
        children={children}
      />
    );
  });
};
interface DataTreeViewProps {
  treeItems: TreeItemData[];
}
function DataTreeView({ treeItems }: DataTreeViewProps) {
  return (
    <TreeView
      defaultCollapseIcon={<ExpandMoreIcon />}
      defaultExpandIcon={<ChevronRightIcon />}
    >
      {getTreeItemsFromData(treeItems)}
    </TreeView>
  );
}

export default function App() {
  return (
    <div className="App">
      <DataTreeView treeItems={sampleFromStackOverflowQuestion} />
      <br />
      <DataTreeView treeItems={seasons} />
    </div>
  );
}
