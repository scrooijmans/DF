# Tree Node support in TypeScript TreeView Control | Syncfusion
28 Sep 202315 minutes to read

TreeView node is structured with expand/collapse arrow, checkbox, image and text as shown in below.

![TypeScript TreeView Tree Node](Tree-Node_images/Tree-Node_img1.png)

Also TreeView node object holds the following properties.


|Properties|Data Type|Description               |
|----------|---------|--------------------------|
|checked   |Boolean  |Checked state of the node |
|count     |Number   |Number of child           |
|expanded  |Boolean  |Expanded state of the node|
|index     |Number   |Index of the node         |
|level     |Number   |Level of the node         |
|id        |String   |Node id                   |
|parentId  |String   |Parent id of the node     |
|selected  |Boolean  |Selected state of the node|


Get/Set Node Value
------------------

TreeView provides a set of options to configure all its properties by setting and getting values at initialization or dynamically.

To get the node value, you can use [getNode](https://help.syncfusion.com/api/js/ejtreeview#methods:getnode) method as shown in the below code example, in which on button click action the node value has retrieved.  
Also you can get the text value of tree node by using [getText](https://help.syncfusion.com/api/js/ejtreeview#methods:gettext) method.

*   JS

```
/// <reference path="tsfiles/jquery.d.ts" />
/// <reference path="tsfiles/ej.web.all.d.ts" />

module TreeViewComponent {
    $(function () {
        var tree = new ej.TreeView($("#treeView"), {

                fields: {
                    dataSource: [

                    {

                        id: 1, text: "Item 1",

                        child: [

                        { id: 4, text: "Item 1.1" },

                        { id: 5, text: "Item 1.2" }

                        ]

                    },

                    { id: 2, text: "Item 2" },

                    { id: 3, text: "Item 3" }

                    ]

                }

            });

        });
  }

        //bind below onClick action to button to get node at button click or else in any action

        function onClick() {
            var node = tree.getNode("1");

        }
```


> Existing TreeView instance can be created by [jQuery.data()](https://api.jquery.com/jQuery.data/#) and you can control the API’s of TreeView behavior.

To edit the node text, you can use [updateText](https://help.syncfusion.com/api/js/ejtreeview#methods:updatetext) method as shown below code example.

*   JS

```
//get node object using getNode method

        var node = tree.getNode("2");

        //sets text to existing node 

        tree.updateText(node.id, "updated Item");
```


Get Parent Node
---------------

To get current parent node of a particular node, you can use the [getParent](https://help.syncfusion.com/api/js/ejtreeview#methods:getparent) method as shown in below code example

*   JS

```
//create an instance from an existing TreeView.

        // only after control creation we can get treeObj otherwise it throws exception.

        var node = tree.getParent("4");
```


Get Node Index
--------------

To get node index you can use the [getNodeIndex](https://help.syncfusion.com/api/js/ejtreeview#methods:getnodeindex) as shown in below code example

You can use [getNodeByIndex](https://help.syncfusion.com/api/js/ejtreeview#methods:getnodebyindex) method to get TreeView node by using index position.

*   JS

```
//create an instance from an existing TreeView.

        var node = tree.getNode("4");

        //gets the index of node 

        tree.getNodeIndex(node.id);
```


Node Manipulations
------------------

You can perform following operation in tree nodes and the modified node values can be saved in database.

### Add or Remove nodes

To add/remove nodes programmatically, use [addNode](https://help.syncfusion.com/api/js/ejtreeview#methods:addnode) and [removeNode](https://help.syncfusion.com/api/js/ejtreeview#methods:removenode) methods of the TreeView.

*   JS

```
//create an instance from an existing TreeView.

        tree.addNode(newNode, "2");

        //to remove node

        tree.removeNode("4");
```


You can able to add a new node after or before specific TreeView node by using [insertAfter](https://help.syncfusion.com/api/js/ejtreeview#methods:insertafter) and [insertBefore](https://help.syncfusion.com/api/js/ejtreeview#methods:insertbefore) methods.

*   JS

```
//create an instance from an existing TreeView.

        var newNode = { id: 12, text: "Item 2.2" };

        //to add tree node after some element, which having id 2

        tree.insertAfter(newNode, "2");

        var newNode = { id: 13, text: "Item 2.3" };

        //to add tree node before some element, which having id 2

        tree.insertBefore(newNode, "2");
```


### Move node

You can also achieve cut and paste operation by using [moveNode](https://help.syncfusion.com/api/js/ejtreeview#methods:movenode) methods.

*   JS

```
//create an instance from an existing TreeView.

        //to move tree node which having id 5, to child of node which having id 2.

        tree.moveNode("5", "2");
```


### Expand or Collapse node

Tree nodes can be expanded or collapsed by clicking the expand/collapse icon of the node or in code behind by using the following methods.


|Methods     |Description                         |
|------------|------------------------------------|
|expandNode  |Expands the node with specified id  |
|collapseNode|Collapse the node with specified id |
|expandAll   |Expands all the node                |
|collapseAll |Collapse all the node               |


Also you can get all the expanded nodes index in tree by using [getExpandedNodesIndex](https://help.syncfusion.com/api/js/ejtreeview#methods:getexpandednodesindex) method, which returns the array of expanded node indices.

### Prevent multiple node expand

You can able to prevent multiple expand of TreeView nodes by specifying [enableMultipleExpand](https://help.syncfusion.com/api/js/ejtreeview#members:enablemultipleexpand) as false.

For example, if you want to allow only one node to be expanded in TreeView at a time. Refer the sample [here](https://jsplayground.syncfusion.com/zdtud5yw).

*   JS

```
// Initialize and bind the TreeView with enableMultipleExpand property

/// <reference path="tsfiles/jquery.d.ts" />
/// <reference path="tsfiles/ej.web.all.d.ts" />

module TreeViewComponent {

    $(function () {
        var tree = new ej.TreeView($("#treeView"), {

                enableMultipleExpand: false,

                dataSource: localData,

                fields: {

                    id: "id",

                    text: "text",

                    parentId: "parent"

                }

            });

        });
}
```


### Get updated node collection

You can get the updated node values after manipulating or editing the node of TreeView to save at server end using [getTreeData](https://help.syncfusion.com/api/js/ejtreeview#methods:gettreedata) method. It returns the JSON data represented as in tree with modified structure.

You can also get the updated data source for remote data binding after performing the operation like editing, selecting/unselecting, expanding/collapsing, checking/unchecking and removing node. You cannot get the updated data source when you perform operation like drag and drop, adding node for remote data binding.

The updated data source also contains custom attributes if you return these from server.

*   JS

```
//create an instance from an existing TreeView.

        //to get TreeView data

        tree.getTreeData();
```


Editing
-------

You can directly edit the tree node’s text in-place by double-click on the tree node or select the tree node and press F2 key. The editing works only if the [allowEditing](https://help.syncfusion.com/api/js/ejtreeview#members:allowediting) property is true in TreeView control. When editing is completed by focus out or “enter” key press, the modified node’s text is saved automatically. The [nodeEdit](https://help.syncfusion.com/api/js/ejtreeview#events:nodeedit) event will be triggered whenever edited the TreeView node.  
Also [beforeEdit](https://help.syncfusion.com/api/js/ejtreeview#events:beforeedit) event will be triggered before the TreeView node change into editing mode.

*   JS

```
// Initialize and bind the TreeView with allowEditing property

/// <reference path="tsfiles/jquery.d.ts" />
/// <reference path="tsfiles/ej.web.all.d.ts" />

module TreeViewComponent {

    $(function () {

        var tree = new ej.TreeView($("#treeView"), {

                allowEditing: true,

                dataSource: localData,

                fields: {

                    id: "id",

                    text: "text",

                    parentId: "parent"

                }

            });

        });
 }
```


Selection
---------

You can select a specific node by setting index value in [selectedNode](https://help.syncfusion.com/api/js/ejtreeview#members:selectednode) property or passing node’s id/element to [selectNode](https://help.syncfusion.com/api/js/ejtreeview#methods:selectnode) method.  
To get the selected status of a given TreeView node you have to use [isSelected](https://help.syncfusion.com/api/js/ejtreeview#methods:isselected) method.  
The [nodeClick](https://help.syncfusion.com/api/js/ejtreeview#events:nodeclick) event will be triggered whenever TreeView node is clicked. The [beforeSelect](https://help.syncfusion.com/api/js/ejtreeview#events:beforeselect) event will be triggered before the TreeView node is selected.  
The [nodeSelect](https://help.syncfusion.com/api/js/ejtreeview#events:nodeselect)/[nodeUnselect](https://help.syncfusion.com/api/js/ejtreeview#events:nodeunselect) events will be triggered based on the TreeView node click operations.

*   JS

```
// Initialize and bind the TreeView with selectedNode property

/// <reference path="tsfiles/jquery.d.ts" />
/// <reference path="tsfiles/ej.web.all.d.ts" />

module TreeViewComponent {

    $(function () {
        var tree = new ej.TreeView($("#treeView"), {

                selectedNode: 2,

                dataSource: localData,

                fields: {

                    id: "id",

                    text: "text",

                    parentId: "parent"

                }

            });

            //create an instance from an existing TreeView.

              //to select node

            tree.selectNode("4");

        });
}
```


Ensure Visibility
-----------------

To get the visibility status of the given TreeView node, you can use [isVisible](https://help.syncfusion.com/api/js/ejtreeview#methods:isvisible) method.  
You can ensure the specified tree node is visible by using [ensureVisible](https://help.syncfusion.com/api/js/ejtreeview#methods:ensurevisible) method, which expands tree nodes and scrolls the TreeView control as necessary.  
Also you can use [getVisibleNodes](https://help.syncfusion.com/api/js/ejtreeview#methods:getvisiblenodes) method to get currently visible nodes in TreeView.

*   JS

```
/// <reference path="tsfiles/jquery.d.ts" />
/// <reference path="tsfiles/ej.web.all.d.ts" />

module TreeViewComponent {

        var localData = [

        { id: 1, text: "Item 1" },

        { id: 2, text: "Item 2" },

        { id: 3, text: "Item 3" },

        { id: 4, text: "Item 4" },

        { id: 5, parent: 1, text: "Item 1.1" },

        { id: 6, parent: 1, text: "Item 1.2" },

        { id: 7, parent: 1, text: "Item 1.3" },

        { id: 8, parent: 3, text: "Item 3.1" },

        { id: 9, parent: 3, text: "Item 3.2" },

        { id: 10, parent: 5, text: "Item 1.1.1" }

        ];

    $(function () {
        var tree = new ej.TreeView($("#treeView"), {

                fields: { dataSource: localData, id: "id", parentId: "parent", text: "text" }

            });

        });
}

        function onClick() {

            //create an instance from an existing TreeView.

            tree.ensureVisible("10");

        }
```
