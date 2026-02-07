from sklearn import tree

# [height, weight, shoe size]

X = [[180, 70, 45], [190, 90, 47], [167, 75, 44], [150, 45, 32], [180, 69, 46], [180, 69, 46], [180, 69, 46], [180, 69, 46], [180, 69, 46], [180, 69, 46], [180, 69, 46]]

Y = ['male', 'male', 'female', 'male', 'male', 'female', 'male', 'male', 'female','male', 'male']
clf = tree.DecisionTreeClassifier()

clf = clf.fit(X, Y)

prediction = clf.predict([[190,89,46]])

print(prediction)
