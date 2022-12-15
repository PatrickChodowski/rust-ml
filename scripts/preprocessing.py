import pandas as pd
import numpy as np
from sklearn.preprocessing import OneHotEncoder, StandardScaler
from sklearn.compose import ColumnTransformer
from sklearn.impute import SimpleImputer

df0 = pd.read_csv("./data/train.csv")
df0.set_index("PassengerId", inplace=True)
cols_to_drop = ['Name', 'Ticket', 'Cabin', 'family', 'Parch', 'SibSp']
cols_to_ohe = ['Embarked', 'Pclass', 'Sex']
cols_to_scale = ['Fare', 'Age']

# special treatment:
df0['No_Cabin'] = np.where(df0['Cabin'].isna(), 1, 0 )
df0['family'] = df0.Parch + df0.SibSp
df0['No_Family'] = np.where(df0['family'] == 0, 1, 0 )

# drop unused
df0.drop(columns=cols_to_drop, inplace=True)

# remove NANs
df0['Age'] = df0['Age'].fillna(df0['Age'].mean())
df0['Fare'] = df0['Age'].fillna(df0['Fare'].mean())
df0['Embarked'] = df0['Embarked'].fillna('S')

# data processing
ct = ColumnTransformer(
    transformers=[
        ("sspip install --upgrade scikit-learn", StandardScaler(), cols_to_scale),
        ("ohe", OneHotEncoder(handle_unknown='ignore', sparse_output=False), cols_to_ohe),
    ], remainder='passthrough', 
       verbose_feature_names_out=False
).set_output(transform='pandas')

df1 = ct.fit_transform(df0)

# Output
print(df1)
print(df1.columns)
df1.to_csv("./data/final_train.csv", index=True)
