#include <iostream>
#include <list>

using namespace std;

class breathfirsttraversal
{
    private:
    int numberOfVertices;
    list <int> * adjacentEdge;
    bool * seen;
    /* data */
public:
    breathfirsttraversal(int verticies);
    void additionOfEdge(int source, int destination);
    void BFS(int stratingVertex);
    // ~breathfirsttraversal();
};

breathfirsttraversal::breathfirsttraversal(int verticies)
{
    numberOfVertices = verticies;
    adjacentEdge = new list<int>[verticies];
}

void breathfirsttraversal::additionOfEdge(int source, int destination){
    adjacentEdge[source].push_back(destination);
    adjacentEdge[destination].push_back(source);
}

void breathfirsttraversal::BFS(int stratingVertex){
    seen = new bool[numberOfVertices];
    for (int i = 0; i < numberOfVertices; i++){
        seen[i] = false;
    }
    list <int> queue;
    seen[stratingVertex] = true;
    queue.push_back(stratingVertex);
    list<int>::iterator i;

    while (!queue.empty()){
        int curVertex = queue.front();
        cout << "Visited" << curVertex << " ";
        queue.pop_front();

        for( i = adjacentEdge[curVertex].begin(); i != adjacentEdge[curVertex].end(); ++i){
            int adjacentVertex = *i;
            if (!seen[adjacentVertex]){
                seen[adjacentVertex] = true;
                queue.push_back(adjacentVertex);
            }
        }

    }

}

int main() {
  breathfirsttraversal v(4);
  v.additionOfEdge(0, 1);
  v.additionOfEdge(0, 2);
  v.additionOfEdge(1, 3);
  v.additionOfEdge(2, 3);
  v.additionOfEdge(2, 4);
  v.additionOfEdge(3, 5);

  v.BFS(2);

  return 0;
}

// breathfirsttraversal::~breathfirsttraversal()
// {
// }
