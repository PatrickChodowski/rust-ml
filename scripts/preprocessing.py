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
df1.reset_index(inplace=True)

# rename and reorder
df1.rename(columns={
  'PassengerId': 'passenger_id',
  'Fare': 'fare',
  'Age': 'age',
  'Embarked_C': 'embarked_c',
  'Embarked_Q': 'embarked_q',
  'Embarked_S': 'embarked_s',
  'Pclass_1': 'pclass1',
  'Pclass_2': 'pclass2',
  'Pclass_3': 'pclass3',
  'Sex_female': 'sex_female',
  'Sex_male': 'sex_male',
  'Survived': 'survived',
  'No_Cabin': 'no_cabin',
  'No_Family': 'no_family'
}, inplace=True)

df1 = df1[['passenger_id', 
           'survived', 
           'fare', 
           'age', 
           'embarked_c', 
           'embarked_q', 
           'embarked_s', 
           'pclass1', 
           'pclass2', 
           'pclass3', 
           'sex_female', 
           'sex_male', 
           'no_cabin', 
           'no_family']]

# Output
print(df1)
print(df1.columns)
df1.to_csv("./data/final_train.csv", index=True)
